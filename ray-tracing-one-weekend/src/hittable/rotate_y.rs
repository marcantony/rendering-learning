use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{NormalizedVec3, Point3, Vec3},
};

use super::{HitRecord, Hittable};

pub struct RotateY<H> {
    object: H,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl<M: Material, H: Hittable<Material = M>> RotateY<H> {
    /// Creates a counter-clockwise rotation around the Y-axis by a number of degrees
    pub fn new(object: H, degrees: f64) -> Self {
        let radians = f64::to_radians(degrees);
        let (sin_theta, cos_theta) = f64::sin_cos(radians);
        let bbox = object.bounding_box();

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut min_z = f64::INFINITY;

        let mut max_x = -f64::INFINITY;
        let mut max_y = -f64::INFINITY;
        let mut max_z = -f64::INFINITY;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;

                    let x = i_f * bbox.x().max + (1.0 - i_f) * bbox.x().min;
                    let y = j_f * bbox.y().max + (1.0 - j_f) * bbox.y().min;
                    let z = k_f * bbox.z().max + (1.0 - k_f) * bbox.z().min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min_x = min_x.min(tester.x());
                    max_x = max_x.max(tester.x());

                    min_y = min_y.min(tester.y());
                    max_y = max_y.max(tester.y());

                    min_z = min_z.min(tester.z());
                    max_z = max_z.max(tester.z());
                }
            }
        }

        let min_point = Point3::new(min_x, min_y, min_z);
        let max_point = Point3::new(max_x, max_y, max_z);

        let rotated_bbox = AABB::from_extrema(&min_point, &max_point);

        RotateY {
            object,
            sin_theta,
            cos_theta,
            bbox: rotated_bbox,
        }
    }
}

impl<M: Material, H: Hittable<Material = M>> Hittable for RotateY<H> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        // Transform incoming ray from world space to object space
        let origin = Point3::new(
            self.cos_theta * r.origin.x() - self.sin_theta * r.origin.z(),
            r.origin.y(),
            self.sin_theta * r.origin.x() + self.cos_theta * r.origin.z(),
        );
        let direction = Point3::new(
            self.cos_theta * r.direction.x() - self.sin_theta * r.direction.z(),
            r.direction.y(),
            self.sin_theta * r.direction.x() + self.cos_theta * r.direction.z(),
        );

        let rotated_r = Ray::new_at_time(origin, direction, r.time);

        // Find an intersection in object space
        self.object.hit(&rotated_r, ray_t).map(|(m, mut hit_rec)| {
            // Transform the intersection back to world space
            hit_rec.p = Point3::new(
                self.cos_theta * hit_rec.p.x() + self.sin_theta * hit_rec.p.z(),
                hit_rec.p.y(),
                -self.sin_theta * hit_rec.p.x() + self.cos_theta * hit_rec.p.z(),
            );

            hit_rec.normal = NormalizedVec3::try_from(Vec3::new(
                self.cos_theta * hit_rec.normal.x() + self.sin_theta * hit_rec.normal.z(),
                hit_rec.normal.y(),
                -self.sin_theta * hit_rec.normal.x() + self.cos_theta * hit_rec.normal.z(),
            ))
            .expect("Instance normal couldn't be normalized after rotation...how?");

            (m, hit_rec)
        })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
