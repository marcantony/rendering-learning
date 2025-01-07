use crate::{
    aabb::AABB,
    hittable::{self, HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{NormalizedVec3, Point3, Vec3},
};

pub struct Quad<M> {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: NormalizedVec3,
    d: f64,
    bbox: AABB,
    material: M,
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

        let n = u.cross(&v);
        let normal = NormalizedVec3::try_from(&n)
            .expect("Failed to find normal because u and v were parallel");
        let d = normal.dot(&q);

        let w = &n / n.dot(&n);

        Quad {
            q,
            u,
            v,
            w,
            normal,
            d,
            bbox,
            material,
        }
    }
}

impl<M: Material> Hittable for Quad<M> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        let denom = self.normal.dot(&r.direction);

        // No hits if the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            // Arbitrary epsilon
            None
        } else {
            // Ray intersects with the plane somewhere
            let t = (self.d - self.normal.dot(&r.origin)) / denom;
            // Return false if hit point is outside the ray interval
            if !ray_t.contains(t) {
                None
            } else {
                // Intersection is in the ray interval
                let intersection = r.at(t);

                let planar_hitpoint_vector = &intersection - &self.q;
                let alpha = self.w.dot(&planar_hitpoint_vector.cross(&self.v));
                let beta = self.w.dot(&self.u.cross(&planar_hitpoint_vector));

                if 0.0 <= alpha && alpha <= 1.0 && 0.0 <= beta && beta <= 1.0 {
                    // Intersection is in the quad
                    let (oriented_normal, face) =
                        hittable::calculate_face_normal(r, self.normal.clone());

                    let record = HitRecord {
                        p: intersection,
                        normal: oriented_normal,
                        t,
                        uv: (alpha, beta),
                        face,
                    };

                    Some((&self.material, record))
                } else {
                    None
                }
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
