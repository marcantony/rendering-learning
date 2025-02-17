use std::{collections::HashMap, io::BufRead, rc::Rc};

use crate::{
    mesh::{FaceDyn, Mesh, Vertex},
    vec3::{Point3, Vec3},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ObjGroup {
    Default,
    Named(String),
}

pub struct WavefrontObj {
    ignored: u32,
    groups: HashMap<ObjGroup, Vec<FaceElement>>,
    vertices: Vec<Point3>,
    normals: Vec<Vec3>,
    texture_coords: Vec<(f64, f64)>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct FaceVertex {
    p_i: usize,
    tc_i: Option<usize>,
    n_i: Option<usize>,
}

#[derive(Debug, PartialEq)]
struct FaceElement {
    face_vertices: Vec<FaceVertex>,
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
        let mut current_group_val = Vec::new();
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
                        let res = parse_face(trimmed);
                        res.map(|ts| current_group_val.push(ts))
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

    pub fn to_mesh(self) -> Mesh<FaceDyn> {
        // Basic heuristic to get a reasonable starting map capacity
        let number_of_faces = self.groups.values().map(Vec::len).sum::<usize>();
        let mut face_vertex_map: HashMap<FaceVertex, Rc<Vertex>> =
            HashMap::with_capacity(number_of_faces);

        let face_elements = self.groups.into_values().flatten();
        let faces = face_elements
            .map(|fe| {
                let vertices = fe
                    .face_vertices
                    .into_iter()
                    .map(|fv| {
                        let vertex_from_map = face_vertex_map.entry(fv).or_insert_with_key(|fv| {
                            let p = self.vertices[fv.p_i - 1].clone();
                            let tc = fv.tc_i.map(|i| self.texture_coords[i - 1].clone());
                            let n = fv.n_i.map(|i| self.normals[i - 1].clone());
                            Rc::new(Vertex {
                                point: p,
                                normal: n,
                                texture_coords: tc,
                            })
                        });
                        Rc::clone(&vertex_from_map)
                    })
                    .collect();
                FaceDyn::new(vertices)
            })
            .collect::<Vec<_>>();

        Mesh { faces }
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

fn parse_face(tail: &str) -> Option<FaceElement> {
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

    indices.map(|is| {
        let face_vertices = is
            .into_iter()
            .map(|(p_i, tc_i, n_i)| FaceVertex { p_i, tc_i, n_i })
            .collect::<Vec<_>>();
        FaceElement { face_vertices }
    })
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
            &FaceElement {
                face_vertices: vec![
                    FaceVertex {
                        p_i: 1,
                        tc_i: None,
                        n_i: Some(3)
                    },
                    FaceVertex {
                        p_i: 2,
                        tc_i: None,
                        n_i: Some(1)
                    },
                    FaceVertex {
                        p_i: 3,
                        tc_i: None,
                        n_i: Some(2)
                    }
                ]
            }
        );

        assert_eq!(
            t2,
            &FaceElement {
                face_vertices: vec![
                    FaceVertex {
                        p_i: 1,
                        tc_i: Some(2),
                        n_i: Some(3)
                    },
                    FaceVertex {
                        p_i: 2,
                        tc_i: Some(3),
                        n_i: Some(1)
                    },
                    FaceVertex {
                        p_i: 3,
                        tc_i: Some(1),
                        n_i: Some(2)
                    }
                ]
            }
        );
    }

    mod to_mesh {
        use crate::mesh::Face;

        use super::*;

        #[test]
        fn to_mesh_basic() {
            let points = vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 1.0),
            ];
            let obj = WavefrontObj {
                ignored: 0,
                groups: HashMap::from([(
                    ObjGroup::Default,
                    vec![FaceElement {
                        face_vertices: vec![
                            FaceVertex {
                                p_i: 1,
                                tc_i: None,
                                n_i: None,
                            },
                            FaceVertex {
                                p_i: 2,
                                tc_i: None,
                                n_i: None,
                            },
                            FaceVertex {
                                p_i: 3,
                                tc_i: None,
                                n_i: None,
                            },
                        ],
                    }],
                )]),
                vertices: points.clone(),
                normals: Vec::new(),
                texture_coords: Vec::new(),
            };

            let mesh = obj.to_mesh();

            assert_eq!(mesh.faces.len(), 1);

            let mesh_points = mesh.faces[0]
                .vertices()
                .iter()
                .map(|v| v.point.clone())
                .collect::<Vec<_>>();
            assert_eq!(mesh_points, points);
        }

        #[test]
        fn common_vertices_shared() {
            let points = vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 1.0),
                Point3::new(0.5, 0.5, 0.5),
            ];
            let normals = vec![
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(-1.0, 0.0, 0.0),
            ];
            let texture_coords = vec![(0.0, 0.0), (0.5, 0.5), (1.0, 1.0), (0.25, 0.75)];
            let obj = WavefrontObj {
                ignored: 0,
                groups: HashMap::from([(
                    ObjGroup::Default,
                    vec![
                        FaceElement {
                            face_vertices: vec![
                                FaceVertex {
                                    p_i: 1,
                                    tc_i: Some(1),
                                    n_i: Some(1),
                                },
                                FaceVertex {
                                    p_i: 2,
                                    tc_i: Some(2),
                                    n_i: Some(2),
                                },
                                FaceVertex {
                                    p_i: 3,
                                    tc_i: Some(3),
                                    n_i: Some(3),
                                },
                            ],
                        },
                        FaceElement {
                            face_vertices: vec![
                                FaceVertex {
                                    p_i: 2,
                                    tc_i: Some(2),
                                    n_i: Some(2),
                                },
                                FaceVertex {
                                    p_i: 3,
                                    tc_i: Some(3),
                                    n_i: None,
                                },
                                FaceVertex {
                                    p_i: 4,
                                    tc_i: Some(4),
                                    n_i: Some(4),
                                },
                            ],
                        },
                    ],
                )]),
                vertices: points.clone(),
                normals: normals.clone(),
                texture_coords: texture_coords.clone(),
            };

            let mesh = obj.to_mesh();

            assert_eq!(mesh.faces.len(), 2);

            let face1 = &mesh.faces[0];
            let face2 = &mesh.faces[1];

            // Face 1 vertex 1 should not be in face 2
            assert!(face2
                .vertices()
                .iter()
                .all(|v| !Rc::ptr_eq(v, &face1.vertices()[0])));
            // Face 1 vertex 3 should not be in face 2
            assert!(face2
                .vertices()
                .iter()
                .all(|v| !Rc::ptr_eq(v, &face1.vertices()[2])));

            // Face 2 vertex 2 should not be in face 1
            assert!(face1
                .vertices()
                .iter()
                .all(|v| !Rc::ptr_eq(v, &face2.vertices()[1])));
            // Face 2 vertex 3 should not be in face 1
            assert!(face1
                .vertices()
                .iter()
                .all(|v| !Rc::ptr_eq(v, &face2.vertices()[2])));

            // Face 1 vertex 2 should be shared with face 2 vertex 1
            assert!(Rc::ptr_eq(&face1.vertices()[1], &face2.vertices()[0]));
        }
    }
}
