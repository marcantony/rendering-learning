use std::{collections::HashMap, io::BufRead};

use crate::{
    math::{point::Point3d, vector::Vec3d},
    scene::object::{bounded::Bounded, group::Group, triangle::Triangle, Object},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ObjGroup {
    Default,
    Named(String),
}

pub struct WavefrontObj {
    ignored: u32,
    groups: HashMap<ObjGroup, Vec<Triangle>>,
    vertices: Vec<Point3d>,
    normals: Vec<Vec3d>,
}

impl WavefrontObj {
    pub fn parse(reader: impl BufRead) -> Self {
        let lines_iter = reader.lines().map(|l| l.unwrap());

        let mut obj = WavefrontObj {
            ignored: 0,
            groups: HashMap::new(),
            vertices: Vec::new(),
            normals: Vec::new(),
        };

        let mut current_group_name = ObjGroup::Default;
        let mut current_group_val = Vec::<Triangle>::new();
        for line in lines_iter {
            let split = line.split_once(' ');
            let status: Option<()> = split.and_then(|(head, tail)| {
                let trimmed = tail.trim();
                match head {
                    "v" => {
                        let res = parse_vertex(trimmed);
                        res.map(|p| obj.vertices.push(p))
                    }
                    "f" => {
                        let res = parse_face(trimmed, &obj.vertices, &obj.normals);
                        res.map(|mut ts| current_group_val.append(&mut ts))
                    }
                    "g" => {
                        let group_vals = current_group_val.drain(..).collect::<Vec<_>>();
                        obj.groups.insert(current_group_name.clone(), group_vals);
                        current_group_name = ObjGroup::Named(String::from(trimmed));
                        current_group_val.clear();
                        Some(())
                    }
                    "vn" => {
                        let res = parse_normal(trimmed);
                        res.map(|n| obj.normals.push(n))
                    }
                    _ => None,
                }
            });

            match status {
                Some(_) => (),
                None => obj.ignored += 1,
            }
        }
        obj.groups.insert(current_group_name, current_group_val);

        obj
    }

    pub fn to_object(self) -> impl Object {
        let all_triangles = self.groups.into_values().flatten().collect::<Vec<_>>();
        Bounded::new(Group::new(all_triangles))
    }
}

fn parse_vertex(tail: &str) -> Option<Point3d> {
    let nums = tail
        .split_whitespace()
        .map(|s| s.parse::<f64>().ok())
        .collect::<Option<Vec<f64>>>();

    nums.and_then(|ns| {
        if ns.len() == 3 {
            Some(Point3d::new(ns[0], ns[1], ns[2]))
        } else {
            None
        }
    })
}

fn parse_normal(tail: &str) -> Option<Vec3d> {
    let nums = tail
        .split_whitespace()
        .map(|s| s.parse::<f64>().ok())
        .collect::<Option<Vec<f64>>>();

    nums.and_then(|ns| {
        if ns.len() == 3 {
            Some(Vec3d::new(ns[0], ns[1], ns[2]))
        } else {
            None
        }
    })
}

fn parse_face(
    tail: &str,
    read_vertices: &[Point3d],
    read_normals: &[Vec3d],
) -> Option<Vec<Triangle>> {
    let tokens = tail.split_whitespace();

    let indices: Option<Vec<(usize, Option<usize>)>> = tokens
        .map(|token| {
            let indices = token.split('/').collect::<Vec<_>>();
            let v_n_indices_unparsed = match indices.len() {
                1 => Some((indices[0], None)),
                2 => Some((indices[0], None)),
                3 => Some((indices[0], Some(indices[2]))),
                _ => None,
            };

            v_n_indices_unparsed.and_then(|(v_unparsed, n_unparsed)| {
                let v_parsed = v_unparsed.parse::<usize>().ok();
                match n_unparsed {
                    Some(val) => {
                        let n_parsed = val.parse::<usize>().ok();
                        n_parsed.and_then(|n| v_parsed.map(|v| (v, Some(n))))
                    }
                    None => v_parsed.map(|v| (v, None)),
                }
            })
        })
        .collect();

    let mapped_indices = indices.map(|ns| {
        ns.iter()
            .map(|(vi, ni)| (&read_vertices[vi - 1], ni.map(|n| &read_normals[n - 1])))
            .collect::<Vec<_>>()
    });

    mapped_indices.and_then(|vertices| {
        if vertices.len() >= 3 {
            let triangulated = fan_triangulate(&vertices);
            Some(
                triangulated
                    .into_iter()
                    .map(|verts| {
                        let points = [verts[0].0, verts[1].0, verts[2].0].map(|p| p.clone());
                        let normals = verts[0].1.and_then(|first| {
                            verts[1].1.and_then(|second| {
                                verts[2]
                                    .1
                                    .map(|third| [first, second, third].map(|n| n.clone()))
                            })
                        });

                        match normals {
                            Some(ns) => {
                                let [p1, p2, p3] = points;
                                let [n1, n2, n3] = ns;
                                Triangle::smooth([(p1, n1), (p2, n2), (p3, n3)], Default::default())
                            }
                            None => Triangle::flat(points, Default::default()),
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    })
}

fn fan_triangulate<'a>(
    vertices: &[(&'a Point3d, Option<&'a Vec3d>)],
) -> Vec<[(&'a Point3d, Option<&'a Vec3d>); 3]> {
    let mut triangles = Vec::<[(&Point3d, Option<&Vec3d>); 3]>::new();

    for i in 2..vertices.len() {
        triangles.push([vertices[0], vertices[i - 1], vertices[i]])
    }

    triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use crate::math::vector::Vec3d;

        use super::*;

        #[test]
        fn ignoring_unrecognized_lines() {
            let gibberish = "There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.";

            let parsed = WavefrontObj::parse(gibberish.as_bytes());

            assert_eq!(parsed.ignored, 5);
        }

        #[test]
        fn vertex_records() {
            let data = "
v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0
";

            let parsed = WavefrontObj::parse(data.as_bytes());

            assert_eq!(parsed.vertices[0], Point3d::new(-1.0, 1.0, 0.0));
            assert_eq!(parsed.vertices[1], Point3d::new(-1.0, 0.5, 0.0));
            assert_eq!(parsed.vertices[2], Point3d::new(1.0, 0.0, 0.0));
            assert_eq!(parsed.vertices[3], Point3d::new(1.0, 1.0, 0.0));
        }

        #[test]
        fn parsing_triangle_faces() {
            let data = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4
";

            let parsed = WavefrontObj::parse(data.as_bytes());

            let g = &parsed.groups.get(&ObjGroup::Default).unwrap();
            let t1 = &g[0];
            let t2 = &g[1];

            assert_eq!(
                t1,
                &Triangle::flat(
                    [
                        parsed.vertices[0].clone(),
                        parsed.vertices[1].clone(),
                        parsed.vertices[2].clone(),
                    ],
                    Default::default()
                )
            );
            assert_eq!(
                t2,
                &Triangle::flat(
                    [
                        parsed.vertices[0].clone(),
                        parsed.vertices[2].clone(),
                        parsed.vertices[3].clone(),
                    ],
                    Default::default()
                )
            );
        }

        #[test]
        fn triangulating_polygons() {
            let data = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0

f 1 2 3 4 5
";

            let parsed = WavefrontObj::parse(data.as_bytes());

            let g = &parsed.groups.get(&ObjGroup::Default).unwrap();
            let t1 = &g[0];
            let t2 = &g[1];
            let t3 = &g[2];

            assert_eq!(
                t1,
                &Triangle::flat(
                    [
                        parsed.vertices[0].clone(),
                        parsed.vertices[1].clone(),
                        parsed.vertices[2].clone(),
                    ],
                    Default::default()
                )
            );
            assert_eq!(
                t2,
                &Triangle::flat(
                    [
                        parsed.vertices[0].clone(),
                        parsed.vertices[2].clone(),
                        parsed.vertices[3].clone(),
                    ],
                    Default::default()
                )
            );
            assert_eq!(
                t3,
                &Triangle::flat(
                    [
                        parsed.vertices[0].clone(),
                        parsed.vertices[3].clone(),
                        parsed.vertices[4].clone(),
                    ],
                    Default::default()
                )
            );
        }

        #[test]
        fn triangles_in_groups() {
            let data = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
";

            let parsed = WavefrontObj::parse(data.as_bytes());

            let g1 = parsed
                .groups
                .get(&ObjGroup::Named(String::from("FirstGroup")))
                .unwrap();
            let g2 = parsed
                .groups
                .get(&ObjGroup::Named(String::from("SecondGroup")))
                .unwrap();
            let t1 = &g1[0];
            let t2 = &g2[0];

            assert_eq!(
                t1,
                &Triangle::flat(
                    [
                        parsed.vertices[0].clone(),
                        parsed.vertices[1].clone(),
                        parsed.vertices[2].clone(),
                    ],
                    Default::default()
                )
            );
            assert_eq!(
                t2,
                &Triangle::flat(
                    [
                        parsed.vertices[0].clone(),
                        parsed.vertices[2].clone(),
                        parsed.vertices[3].clone(),
                    ],
                    Default::default()
                )
            );
        }

        #[test]
        fn vertex_normal_records() {
            let data = "
vn 0 0 1
vn 0.707 0 -0.707
vn 1 2 3
";

            let parsed = WavefrontObj::parse(data.as_bytes());

            assert_eq!(
                parsed.normals,
                [
                    Vec3d::new(0.0, 0.0, 1.0),
                    Vec3d::new(0.707, 0.0, -0.707),
                    Vec3d::new(1.0, 2.0, 3.0),
                ]
            )
        }

        #[test]
        fn faces_with_normals() {
            let data = "
v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

f 1//3 2//1 3//2
f 1/0/3 2/102/1 3/14/2
";

            let parsed = WavefrontObj::parse(data.as_bytes());

            let g = parsed.groups.get(&ObjGroup::Default).unwrap();
            let t1 = &g[0];
            let t2 = &g[1];

            assert_eq!(
                t1,
                &Triangle::smooth(
                    [
                        (Point3d::new(0.0, 1.0, 0.0), Vec3d::new(0.0, 1.0, 0.0)),
                        (Point3d::new(-1.0, 0.0, 0.0), Vec3d::new(-1.0, 0.0, 0.0)),
                        (Point3d::new(1.0, 0.0, 0.0), Vec3d::new(1.0, 0.0, 0.0)),
                    ],
                    Default::default()
                )
            );
            assert_eq!(t1, t2);
        }
    }

    #[test]
    fn converting_parsed_obj_file_to_group() {
        let data = "
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3
g SecondGroup
f 1 3 4
";

        let parsed = WavefrontObj::parse(data.as_bytes());
        parsed.to_object(); // At least make sure it doesn't panic...
    }
}
