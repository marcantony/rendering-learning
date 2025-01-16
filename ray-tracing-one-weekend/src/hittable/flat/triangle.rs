use crate::{
    aabb::AABB,
    hittable::{self, HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{NormalizedVec3, Point3, Vec3},
};

use super::plane::Plane;

#[derive(Clone)]
pub struct Triangle<M> {
    plane: Plane<M>,
    bbox: AABB,
    normals: Option<[Vec3; 3]>,
    texture_coords: Option<[(f64, f64); 3]>,
}

impl<M> Triangle<M> {
    /// Creates a flat-shaded triangle with point q representing the starting
    /// corner and vectors u and v representing the first and second sides,
    /// both originating from the starting corner.
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: M) -> Self {
        let p2 = &q + &u;
        let p3 = &q + &v;
        Triangle::from_model([q, p2, p3], None, None, material)
    }

    /// Creates a triangle spanning 3 points, with optional texture coords and normals
    /// at each vertex.
    pub fn from_model(
        points: [Point3; 3],
        texture_coords: Option<[(f64, f64); 3]>,
        normals: Option<[Vec3; 3]>,
        material: M,
    ) -> Self {
        let [p1, p2, p3] = points;

        // Compute the bounding box of all vertices
        let bbox = AABB::from_points(&[&p1, &p2, &p3]);

        let u = &p2 - &p1;
        let v = &p3 - &p1;

        let plane = Plane::new(p1, u, v, material);

        Self {
            plane,
            bbox,
            normals,
            texture_coords,
        }
    }
}

impl<M: Material> Hittable for Triangle<M> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        self.plane
            .hit(r, ray_t)
            .filter(|hit| {
                let (alpha, beta) = hit.1.uv;
                0.0 <= alpha && 0.0 <= beta && alpha + beta <= 1.0
            })
            .map(|mut hit| {
                let (u, v) = hit.1.uv;
                let frac2 = u;
                let frac3 = v;
                let frac1 = 1.0 - u - v;

                match &self.normals {
                    Some([v1, v2, v3]) => {
                        let normal =
                            NormalizedVec3::try_from(v2 * frac2 + v3 * frac3 + v1 * frac1).unwrap();
                        let (oriented_normal, face) = hittable::calculate_face_normal(&r, normal);
                        hit.1.normal = oriented_normal;
                        hit.1.face = face;
                    }
                    None => (),
                };

                match &self.texture_coords {
                    Some([t1, t2, t3]) => {
                        let new_u = t1.0 * frac1 + t2.0 * frac2 + t3.0 * frac3;
                        let new_v = t1.1 * frac1 + t2.1 * frac2 + t3.1 * frac3;
                        hit.1.uv = (new_u, new_v)
                    }
                    None => (),
                };

                hit
            })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
