use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::plane::Plane;

#[derive(Clone)]
pub struct Triangle<M> {
    plane: Plane<M>,
    bbox: AABB,
}

impl<M> Triangle<M> {
    /// Creates a triangle with point q representing the starting corner
    /// and vectors u and v representing the first and second sides, both
    /// originating from the starting corner.
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: M) -> Self {
        // Compute the bounding box of all vertices
        let bbox = AABB::from_points(&[&q, &(&q + &u), &(&q + &v)]);

        let plane = Plane::new(q, u, v, material);

        Self { plane, bbox }
    }
}

impl<M: Material> Hittable for Triangle<M> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        self.plane.hit(r, ray_t).filter(|hit| {
            let (alpha, beta) = hit.1.uv;
            0.0 <= alpha && 0.0 <= beta && alpha + beta <= 1.0
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
