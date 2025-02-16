use std::{rc::Rc, usize};

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
pub trait Face {
    /// The vertices composing the face. Vertices are stored in [Rc] allowing for shared ownership.
    /// A given vertex is assumed to be shared between two faces geometrically if and only if both
    /// faces share ownership of it.
    fn vertices(&self) -> &[Rc<Vertex>];
}

pub struct FaceN<const N: usize> {
    vertices: [Rc<Vertex>; N],
}
impl<const N: usize> FaceN<N> {
    pub fn new(vertices: [Rc<Vertex>; N]) -> Self {
        assert!(N >= 3, "Polygons must have at least 3 vertices");
        FaceN { vertices }
    }
}
impl<const N: usize> Face for FaceN<N> {
    fn vertices(&self) -> &[Rc<Vertex>] {
        &self.vertices
    }
}
impl<const N: usize> TryFrom<FaceDyn> for FaceN<N> {
    type Error = String;

    fn try_from(value: FaceDyn) -> Result<Self, Self::Error> {
        let len = value.vertices.len();
        Ok(FaceN {
            vertices: value
                .vertices
                .try_into()
                .map_err(|_| format!("Face has {len} vertices but N={N}"))?,
        })
    }
}

pub struct FaceDyn {
    vertices: Vec<Rc<Vertex>>,
}
impl FaceDyn {
    pub fn new(vertices: Vec<Rc<Vertex>>) -> Self {
        assert!(
            vertices.len() >= 3,
            "Polygons must have at least 3 vertices"
        );
        FaceDyn { vertices }
    }
}
impl Face for FaceDyn {
    fn vertices(&self) -> &[Rc<Vertex>] {
        &self.vertices
    }
}
impl<const N: usize> From<FaceN<N>> for FaceDyn {
    fn from(value: FaceN<N>) -> Self {
        FaceDyn {
            vertices: value.vertices.into(),
        }
    }
}

pub struct Mesh<F: Face> {
    pub faces: Vec<F>,
}

impl<F: Face> Mesh<F> {
    pub fn triangulate(self) -> Mesh<FaceN<3>> {
        let new_faces = self
            .faces
            .into_iter()
            .flat_map(|f| fan_triangulate(f))
            .collect::<Vec<_>>();

        Mesh { faces: new_faces }
    }
}

impl Mesh<FaceN<3>> {
    pub fn to_hittable<'a, M: Material + ?Sized>(
        &self,
        material: &'a M,
    ) -> impl Hittable<Material = &'a M> {
        let all_triangles = self
            .faces
            .iter()
            .map(|f| {
                let points = f
                    .vertices()
                    .iter()
                    .map(|v| v.point.clone())
                    .collect::<Vec<_>>();
                let normals = f
                    .vertices()
                    .iter()
                    .map(|v| v.normal.clone())
                    .collect::<Option<Vec<_>>>();
                let texture_coords = f
                    .vertices()
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
}

impl<const N: usize> TryFrom<Mesh<FaceDyn>> for Mesh<FaceN<N>> {
    type Error = String;

    fn try_from(value: Mesh<FaceDyn>) -> Result<Self, Self::Error> {
        let (faces, errors): (Vec<_>, Vec<_>) = value
            .faces
            .into_iter()
            .map(|f| TryInto::<FaceN<N>>::try_into(f))
            .enumerate()
            .map(|(i, r)| r.map_err(|s| format!("Face {i}: {s}")))
            .partition(Result::is_ok);

        let errors: Vec<String> = errors
            .into_iter()
            .map(|r| r.err().unwrap_or_else(|| panic!("oh no")))
            .collect();
        if !errors.is_empty() {
            let message = errors.join("; ");
            Err(message)
        } else {
            let faces: Vec<FaceN<N>> = faces.into_iter().map(Result::unwrap).collect();
            Ok(Mesh { faces })
        }
    }
}

impl<const N: usize> From<Mesh<FaceN<N>>> for Mesh<FaceDyn> {
    fn from(value: Mesh<FaceN<N>>) -> Self {
        Mesh {
            faces: value.faces.into_iter().map(|f| f.into()).collect(),
        }
    }
}

fn fan_triangulate<F: Face>(face: F) -> Vec<FaceN<3>> {
    let mut faces = Vec::new();
    for i in 2..face.vertices().len() {
        faces.push(FaceN::new([
            Rc::clone(&face.vertices()[0]),
            Rc::clone(&face.vertices()[i - 1]),
            Rc::clone(&face.vertices()[i]),
        ]));
    }
    faces
}

#[cfg(test)]
mod tests {
    use crate::{interval::Interval, material::Flat, ray::Ray};

    use super::*;

    mod face {
        use super::*;

        #[test]
        #[should_panic(expected = "3 vertices")]
        fn constructing_face_with_less_than_three_vertices_facen() {
            let v = Vertex {
                point: Point3::new(0.0, 0.0, 0.0),
                normal: None,
                texture_coords: None,
            };

            FaceN::new([Rc::new(v)]);
        }

        #[test]
        #[should_panic(expected = "3 vertices")]
        fn constructing_face_with_less_than_three_vertices_facedyn() {
            let v = Vertex {
                point: Point3::new(0.0, 0.0, 0.0),
                normal: None,
                texture_coords: None,
            };

            FaceDyn::new(vec![Rc::new(v)]);
        }
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
        let face = FaceN::new(vertices);
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

        let face = FaceN::new(vertices.map(|v| Rc::new(v)));
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
