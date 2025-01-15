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
}

impl<M> Triangle<M> {
    /// Creates a flat-shaded triangle with point q representing the starting
    /// corner and vectors u and v representing the first and second sides,
    /// both originating from the starting corner.
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: M) -> Self {
        let p2 = &q + &u;
        let p3 = &q + &v;
        Triangle::flat([q, p2, p3], material)
    }

    /// Creates a flat-shaded triangle spanning the given points.
    pub fn flat(points: [Point3; 3], material: M) -> Self {
        let [p1, p2, p3] = points;

        // Compute the bounding box of all vertices
        let bbox = AABB::from_points(&[&p1, &p2, &p3]);

        let u = &p2 - &p1;
        let v = &p3 - &p1;

        let plane = Plane::new(p1, u, v, material);

        Self {
            plane,
            bbox,
            normals: None,
        }
    }

    /// Creates a smooth-shaded triangle spanning the given points with the
    /// given normal vectors at each point.
    pub fn smooth(vertices: [(Point3, Vec3); 3], material: M) -> Self {
        let [(p1, v1), (p2, v2), (p3, v3)] = vertices;

        // Compute the bounding box of all vertices
        let bbox = AABB::from_points(&[&p1, &p2, &p3]);

        let u = &p2 - &p1;
        let v = &p3 - &p1;

        let plane = Plane::new(p1, u, v, material);

        Self {
            plane,
            bbox,
            normals: Some([v1, v2, v3]),
        }
    }
}

impl<M: Material> Hittable for Triangle<M> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        let triangle_hit = self.plane.hit(r, ray_t).filter(|hit| {
            let (alpha, beta) = hit.1.uv;
            0.0 <= alpha && 0.0 <= beta && alpha + beta <= 1.0
        });

        triangle_hit.map(|mut hit| match &self.normals {
            Some([v1, v2, v3]) => {
                let (u, v) = hit.1.uv;
                let normal =
                    NormalizedVec3::try_from(v2 * u + v3 * v + v1 * (1.0 - u - v)).unwrap();
                let (oriented_normal, face) = hittable::calculate_face_normal(&r, normal);
                hit.1.normal = oriented_normal;
                hit.1.face = face;
                hit
            }
            None => hit,
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
