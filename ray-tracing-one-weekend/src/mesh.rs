use std::rc::Rc;

use crate::{
    bvh::Bvh,
    hittable::{flat::triangle::Triangle, Hittable},
    material::Material,
    vec3::{Point3, Vec3},
};

pub struct Vertex {
    pub point: Point3,
    pub normal: Option<Vec3>,
    pub texture_coords: Option<(f64, f64)>,
}

/// Represents a single face of a mesh. The vertices of the face should be planar.
/// Behavior for nonplanar faces is undefined.
pub struct Face {
    /// The vertices composing the face. Vertices are stored in [Rc] allowing for shared ownership.
    /// A given vertex is assumed to be shared between two faces geometrically if and only if both
    /// faces share ownership of it.
    pub vertices: Vec<Rc<Vertex>>,
}

pub struct Mesh {
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn to_hittable<'a, M: Material + ?Sized>(
        &self,
        material: &'a M,
    ) -> impl Hittable<Material = &'a M> {
        assert!(
            self.faces.iter().all(|f| f.vertices.len() == 3),
            "Only a triangulated mesh can be turned into a hittable"
        );
        let all_triangles = self
            .faces
            .iter()
            .map(|f| {
                let points = f
                    .vertices
                    .iter()
                    .map(|v| v.point.clone())
                    .collect::<Vec<_>>();
                let normals = f
                    .vertices
                    .iter()
                    .map(|v| v.normal.clone())
                    .collect::<Option<Vec<_>>>();
                let texture_coords = f
                    .vertices
                    .iter()
                    .map(|v| v.texture_coords.clone())
                    .collect::<Option<Vec<_>>>();

                let ps: [Point3; 3] = points.try_into().unwrap();
                let ns: Option<[Vec3; 3]> = normals.map(|n| n.try_into().unwrap());
                let tcs: Option<[(f64, f64); 3]> = texture_coords.map(|tc| tc.try_into().unwrap());

                Triangle::from_model(ps, tcs, ns, material)
            })
            .collect::<Vec<_>>();
        Bvh::new(all_triangles)
    }

    pub fn triangulate(self) -> Self {
        let new_faces = self
            .faces
            .into_iter()
            .flat_map(|f| fan_triangulate(f))
            .collect::<Vec<_>>();

        Mesh { faces: new_faces }
    }
}

fn fan_triangulate(face: Face) -> Vec<Face> {
    let mut faces = Vec::new();
    for i in 2..face.vertices.len() {
        faces.push(Face {
            vertices: vec![
                Rc::clone(&face.vertices[0]),
                Rc::clone(&face.vertices[i - 1]),
                Rc::clone(&face.vertices[i]),
            ],
        });
    }
    faces
}

#[cfg(test)]
mod tests {
    use crate::{interval::Interval, material::Flat, ray::Ray};

    use super::*;

    #[test]
    #[should_panic(expected = "triangulated mesh")]
    fn non_triangulated_mesh_to_hittable() {
        let points = (0..=7)
            .map(|i| i as f64 * 3.0)
            .map(|i| [i, i + 1.0, i + 2.0])
            .map(|[i1, i2, i3]| Point3::new(i1, i2, i3));
        let mut vertices = points
            .map(|p| {
                Rc::new(Vertex {
                    point: p,
                    normal: None,
                    texture_coords: None,
                })
            })
            .collect::<Vec<_>>();

        let face2_vertices = vertices.split_off(3);
        let face1_vertices = vertices;

        let face1 = Face {
            vertices: face1_vertices,
        };
        let face2 = Face {
            vertices: face2_vertices,
        };

        let mesh = Mesh {
            faces: vec![face1, face2],
        };

        let material = Flat;
        mesh.to_hittable(&material);
    }

    #[test]
    fn creates_correct_hittable() {
        let points = [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
        ];
        let vertices = points.map(|p| {
            Rc::new(Vertex {
                point: p,
                normal: None,
                texture_coords: None,
            })
        });
        let face = Face {
            vertices: Vec::from(vertices),
        };
        let mesh = Mesh { faces: vec![face] };
        let hittable = mesh.to_hittable(&Flat);

        let should_hit = Ray::new(Point3::new(0.25, 1.0, 0.25), Vec3::new(0.0, -1.0, 0.0));
        let should_miss = Ray::new(Point3::new(0.75, 1.0, 0.75), Vec3::new(0.0, -1.0, 0.0));

        assert!(hittable.hit(&should_hit, &Interval::universe()).is_some());
        assert!(hittable.hit(&should_miss, &Interval::universe()).is_none());
    }

    #[test]
    fn triangulation() {
        let points = [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
        ];

        let vertices = points.clone().map(|p| Vertex {
            point: p,
            normal: None,
            texture_coords: None,
        });

        let face = Face {
            vertices: vertices.map(|v| Rc::new(v)).into(),
        };
        let mesh = Mesh { faces: vec![face] };

        let triangulated = mesh.triangulate();
        let points_by_face = triangulated
            .faces
            .iter()
            .map(|f| f.vertices.iter().map(|v| &v.point).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let expected_points = vec![
            vec![&points[0], &points[1], &points[2]],
            vec![&points[0], &points[2], &points[3]],
        ];

        assert_eq!(points_by_face, expected_points);
    }
}
