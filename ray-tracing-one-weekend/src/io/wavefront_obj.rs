use std::{collections::HashMap, io::BufRead};

use crate::{
    bvh::Bvh,
    hittable::flat::triangle::Triangle,
    material::Material,
    vec3::{Point3, Vec3},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ObjGroup {
    Default,
    Named(String),
}

pub struct WavefrontObj {
    ignored: u32,
    groups: HashMap<ObjGroup, Vec<Tri>>,
    vertices: Vec<Point3>,
    normals: Vec<Vec3>,
    texture_coords: Vec<(f64, f64)>,
}

#[derive(Debug, PartialEq)]
struct Tri {
    points: [Point3; 3],
    texture_coords: Option<[(f64, f64); 3]>,
    normals: Option<[Vec3; 3]>,
}

impl WavefrontObj {
    pub fn parse(reader: impl BufRead) -> Self {
        let lines_iter = reader.lines().map(|l| l.unwrap());

        let mut obj = WavefrontObj {
            ignored: 0,
            groups: HashMap::new(),
            vertices: Vec::new(),
            normals: Vec::new(),
            texture_coords: Vec::new(),
        };

        let mut current_group_name = ObjGroup::Default;
        let mut current_group_val = Vec::<Tri>::new();
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
                        let res =
                            parse_face(trimmed, &obj.vertices, &obj.texture_coords, &obj.normals);
                        res.map(|mut ts| current_group_val.append(&mut ts))
                    }
                    "g" => {
                        let group_vals = std::mem::take(&mut current_group_val);
                        obj.groups.insert(current_group_name.clone(), group_vals);
                        current_group_name = ObjGroup::Named(String::from(trimmed));
                        current_group_val.clear();
                        Some(())
                    }
                    "vn" => {
                        let res = parse_normal(trimmed);
                        res.map(|n| obj.normals.push(n))
                    }
                    "vt" => {
                        let res = parse_texture_coord(trimmed);
                        res.map(|tc| obj.texture_coords.push(tc))
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

    pub fn to_object<'a, M: Material + ?Sized>(self, material: &'a M) -> Bvh<Triangle<&'a M>> {
        let all_triangles = self
            .groups
            .into_values()
            .flatten()
            .map(|t| {
                let Tri {
                    points,
                    texture_coords,
                    normals,
                } = t;
                Triangle::from_model(points, texture_coords, normals, material)
            })
            .collect::<Vec<_>>();
        Bvh::new(all_triangles)
    }
}

fn parse_texture_coord(tail: &str) -> Option<(f64, f64)> {
    let nums = tail
        .split_whitespace()
        .map(|s| s.parse::<f64>().ok())
        .collect::<Option<Vec<f64>>>();

    nums.and_then(|ns| match ns.len() {
        0 => None,
        1 => Some((ns[0], 0.0)),
        _ => Some((ns[0], ns[1])),
    })
}

fn parse_vertex(tail: &str) -> Option<Point3> {
    let nums = tail
        .split_whitespace()
        .map(|s| s.parse::<f64>().ok())
        .collect::<Option<Vec<f64>>>();

    nums.and_then(|ns| {
        if ns.len() == 3 {
            Some(Point3::new(ns[0], ns[1], ns[2]))
        } else {
            None
        }
    })
}

fn parse_normal(tail: &str) -> Option<Vec3> {
    let nums = tail
        .split_whitespace()
        .map(|s| s.parse::<f64>().ok())
        .collect::<Option<Vec<f64>>>();

    nums.and_then(|ns| {
        if ns.len() == 3 {
            Some(Vec3::new(ns[0], ns[1], ns[2]))
        } else {
            None
        }
    })
}

fn parse_face(
    tail: &str,
    read_vertices: &[Point3],
    read_texcoords: &[(f64, f64)],
    read_normals: &[Vec3],
) -> Option<Vec<Tri>> {
    let tokens = tail.split_whitespace();

    let indices: Option<Vec<(usize, Option<usize>, Option<usize>)>> = tokens
        .map(|token| {
            let indices = token.split('/').collect::<Vec<_>>();
            let v_t_n_indices_unparsed = match indices.len() {
                1 => Some((indices[0], None, None)),
                2 => Some((indices[0], Some(indices[1]), None)),
                3 => Some((indices[0], Some(indices[1]), Some(indices[2]))),
                _ => None,
            };

            v_t_n_indices_unparsed.and_then(|(v_unparsed, t_unparsed, n_unparsed)| {
                let v_parsed = v_unparsed.parse::<usize>().ok();
                let t_parsed = t_unparsed.and_then(|t| {
                    if t.is_empty() {
                        None
                    } else {
                        t.parse::<usize>().ok()
                    }
                });
                let n_parsed = n_unparsed.and_then(|n| n.parse::<usize>().ok());

                v_parsed.map(|v| (v, t_parsed, n_parsed))
            })
        })
        .collect();

    let mapped_indices = indices.map(|ns| {
        ns.iter()
            .map(|(vi, ti, ni)| {
                (
                    &read_vertices[vi - 1],
                    ti.map(|t| &read_texcoords[t - 1]),
                    ni.map(|n| &read_normals[n - 1]),
                )
            })
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
                        let texture_coords = verts[0].1.and_then(|first| {
                            verts[1].1.and_then(|second| {
                                verts[2]
                                    .1
                                    .map(|third| [first, second, third].map(|n| n.clone()))
                            })
                        });
                        let normals = verts[0].2.and_then(|first| {
                            verts[1].2.and_then(|second| {
                                verts[2]
                                    .2
                                    .map(|third| [first, second, third].map(|n| n.clone()))
                            })
                        });

                        Tri {
                            points,
                            texture_coords,
                            normals,
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
    vertices: &[(&'a Point3, Option<&'a (f64, f64)>, Option<&'a Vec3>)],
) -> Vec<[(&'a Point3, Option<&'a (f64, f64)>, Option<&'a Vec3>); 3]> {
    let mut triangles = Vec::<[(&'a Point3, Option<&'a (f64, f64)>, Option<&'a Vec3>); 3]>::new();

    for i in 2..vertices.len() {
        triangles.push([vertices[0], vertices[i - 1], vertices[i]])
    }

    triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn faces_with_normals() {
        let data = "
v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

vt 1 2
vt 4 5
vt 7 8

f 1//3 2//1 3//2
f 1/2/3 2/3/1 3/1/2
";

        let parsed = WavefrontObj::parse(data.as_bytes());

        let g = parsed.groups.get(&ObjGroup::Default).unwrap();
        let t1 = &g[0];
        let t2 = &g[1];

        assert_eq!(
            t1,
            &Tri {
                points: [
                    Point3::new(0.0, 1.0, 0.0),
                    Point3::new(-1.0, 0.0, 0.0),
                    Point3::new(1.0, 0.0, 0.0)
                ],
                texture_coords: None,
                normals: Some([
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(-1.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0)
                ])
            }
        );

        assert_eq!(
            t2,
            &Tri {
                points: [
                    Point3::new(0.0, 1.0, 0.0),
                    Point3::new(-1.0, 0.0, 0.0),
                    Point3::new(1.0, 0.0, 0.0)
                ],
                texture_coords: Some([(4.0, 5.0), (7.0, 8.0), (1.0, 2.0)]),
                normals: Some([
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(-1.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0)
                ])
            }
        );
    }
}
