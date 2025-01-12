use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    matrix::Matrix3,
    ray::Ray,
    vec3::{NormalizedVec3, Point3},
};

use super::{HitRecord, Hittable};

pub struct Transform<H> {
    object: H,
    bbox: AABB,
    transformation: Matrix3,
    inv_transformation: Matrix3,
    inv_transpose_transformation: Matrix3,
}

impl<M: Material, H: Hittable<Material = M>> Transform<H> {
    pub fn rotate_x(object: H, degrees: f64) -> Self {
        let radians = f64::to_radians(degrees);
        let (sin_theta, cos_theta) = f64::sin_cos(radians);

        let transformation = Matrix3([
            [1.0, 0.0, 0.0],
            [0.0, cos_theta, -sin_theta],
            [0.0, sin_theta, cos_theta],
        ]);
        let inv_transformation = Matrix3([
            [1.0, 0.0, 0.0],
            [0.0, cos_theta, sin_theta],
            [0.0, -sin_theta, cos_theta],
        ]);

        Transform::new(object, transformation, inv_transformation)
    }

    pub fn rotate_y(object: H, degrees: f64) -> Self {
        let radians = f64::to_radians(degrees);
        let (sin_theta, cos_theta) = f64::sin_cos(radians);

        let transformation = Matrix3([
            [cos_theta, 0.0, sin_theta],
            [0.0, 1.0, 0.0],
            [-sin_theta, 0.0, cos_theta],
        ]);
        let inv_transformation = Matrix3([
            [cos_theta, 0.0, -sin_theta],
            [0.0, 1.0, 0.0],
            [sin_theta, 0.0, cos_theta],
        ]);

        Transform::new(object, transformation, inv_transformation)
    }

    pub fn rotate_z(object: H, degrees: f64) -> Self {
        let radians = f64::to_radians(degrees);
        let (sin_theta, cos_theta) = f64::sin_cos(radians);

        let transformation = Matrix3([
            [cos_theta, -sin_theta, 0.0],
            [sin_theta, cos_theta, 0.0],
            [0.0, 0.0, 1.0],
        ]);
        let inv_transformation = Matrix3([
            [cos_theta, sin_theta, 0.0],
            [-sin_theta, cos_theta, 0.0],
            [0.0, 0.0, 1.0],
        ]);

        Transform::new(object, transformation, inv_transformation)
    }

    pub fn scale(object: H, scale: f64) -> Self {
        let transformation = Matrix3([[scale, 0.0, 0.0], [0.0, scale, 0.0], [0.0, 0.0, scale]]);
        let inv_scale = 1.0 / scale;
        let inv_transformation = Matrix3([
            [inv_scale, 0.0, 0.0],
            [0.0, inv_scale, 0.0],
            [0.0, 0.0, inv_scale],
        ]);

        Transform::new(object, transformation, inv_transformation)
    }

    fn new(object: H, transformation: Matrix3, inv_transformation: Matrix3) -> Self {
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

                    let point_to_rotate = Point3::new(x, y, z);
                    let tester = &transformation * &point_to_rotate;

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

        let inv_transpose_transformation = inv_transformation.transpose();

        Transform {
            object,
            transformation,
            inv_transformation,
            inv_transpose_transformation,
            bbox: rotated_bbox,
        }
    }
}

impl<M: Material, H: Hittable<Material = M>> Hittable for Transform<H> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        // Transform incoming ray from world space to object space
        let origin = &self.inv_transformation * &r.origin;
        let direction = &self.inv_transformation * &r.direction;
        let transformed_r = Ray::new_at_time(origin, direction, r.time);

        // Find an intersection in object space
        self.object
            .hit(&transformed_r, ray_t)
            .map(|(m, mut hit_rec)| {
                // Transform the intersection back to world space
                hit_rec.p = &self.transformation * &hit_rec.p;

                hit_rec.normal =
                    NormalizedVec3::try_from(&self.inv_transpose_transformation * &*hit_rec.normal)
                        .expect("Instance normal couldn't be normalized after rotation...how?");

                (m, hit_rec)
            })
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
