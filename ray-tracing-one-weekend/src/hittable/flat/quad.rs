use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::plane::Plane;

pub struct Quad<M> {
    plane: Plane<M>,
    bbox: AABB,
}

impl<M> Quad<M> {
    /// Creates a parallelogram with point q representing the starting corner
    /// and vectors u and v representing the first and second sides, both
    /// originating from the starting corner.
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: M) -> Self {
        // Compute the bounding box of all four vertices
        let bbox_diagonal1 = AABB::new_from_points(&q, &(&q + &u + &v));
        let bbox_diagonal2 = AABB::new_from_points(&(&q + &u), &(&q + &v));
        let bbox = bbox_diagonal1.merge(&bbox_diagonal2);

        let plane = Plane::new(q, u, v, material);

        Quad { plane, bbox }
    }
}

impl<M: Material> Hittable for Quad<M> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        self.plane.hit(r, ray_t).filter(|hit| {
            let (alpha, beta) = hit.1.uv;
            0.0 <= alpha && alpha <= 1.0 && 0.0 <= beta && beta <= 1.0
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
