use std::{collections::HashMap, io::BufRead};

use crate::{
    bvh::Bvh,
    hittable::{flat::triangle::Triangle, Hittable},
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
}

enum Tri {
    Smooth([(Point3, Vec3); 3]),
    Flat([Point3; 3]),
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
                        let res = parse_face(trimmed, &obj.vertices, &obj.normals);
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

    pub fn to_object<'a, M: Material>(self, material: &'a M) -> impl Hittable<Material = &'a M> {
        let all_triangles = self
            .groups
            .into_values()
            .flatten()
            .map(|t| match t {
                Tri::Flat([q, a, b]) => {
                    let u = &a - &q;
                    let v = &b - &q;
                    Triangle::new(q, u, v, material)
                }
                Tri::Smooth(_) => todo!(),
            })
            .collect::<Vec<_>>();
        Bvh::new(all_triangles)
    }
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

fn parse_face(tail: &str, read_vertices: &[Point3], read_normals: &[Vec3]) -> Option<Vec<Tri>> {
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
                                Tri::Smooth([(p1, n1), (p2, n2), (p3, n3)])
                            }
                            None => Tri::Flat(points),
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
    vertices: &[(&'a Point3, Option<&'a Vec3>)],
) -> Vec<[(&'a Point3, Option<&'a Vec3>); 3]> {
    let mut triangles = Vec::<[(&Point3, Option<&Vec3>); 3]>::new();

    for i in 2..vertices.len() {
        triangles.push([vertices[0], vertices[i - 1], vertices[i]])
    }

    triangles
}
