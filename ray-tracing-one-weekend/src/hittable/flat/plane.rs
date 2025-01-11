use crate::{
    aabb::AABB,
    hittable::{self, HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{NormalizedVec3, Point3, Vec3},
};

pub struct Plane<M> {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: NormalizedVec3,
    d: f64,
    material: M,
}

impl<M> Plane<M> {
    /// Creates a plane containing point q with the basis vectors u and v.
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: M) -> Self {
        let n = u.cross(&v);
        let normal = NormalizedVec3::try_from(&n)
            .expect("Failed to find normal because u and v were parallel");
        let d = normal.dot(&q);

        let w = &n / n.dot(&n);

        Plane {
            q,
            u,
            v,
            w,
            normal,
            d,
            material,
        }
    }
}

pub struct PlaneIntersection {
    alpha: f64,
    beta: f64,
    intersection: Point3,
    t: f64,
}

impl<M: Material> Plane<M> {
    fn hit_ab(&self, r: &Ray, ray_t: &Interval) -> Option<PlaneIntersection> {
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

                Some(PlaneIntersection {
                    alpha,
                    beta,
                    intersection,
                    t,
                })
            }
        }
    }
}

impl<M: Material> Hittable for Plane<M> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        self.hit_ab(r, ray_t).map(|pi| {
            let (oriented_normal, face) = hittable::calculate_face_normal(r, self.normal.clone());

            let record = HitRecord {
                p: pi.intersection,
                normal: oriented_normal,
                t: pi.t,
                uv: (pi.alpha, pi.beta),
                face,
            };

            (&self.material, record)
        })
    }

    fn bounding_box(&self) -> AABB {
        AABB::universe()
    }
}
