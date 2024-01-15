use crate::{
    math::{matrix::InvertibleMatrix, point::Point3d, vector::NormalizedVec3d},
    scene::{material::Material, ray::Ray},
};

use super::Object;

/// A cylinder, by default with radius 1 and infinite length around the y-axis
pub struct Cylinder {
    pub material: Material,
    pub transform: InvertibleMatrix<4>,
}

impl Object for Cylinder {
    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &InvertibleMatrix<4> {
        &self.transform
    }

    fn intersect_local(&self, object_ray: &Ray) -> Vec<f64> {
        let a = object_ray.direction.x().powi(2) + object_ray.direction.z().powi(2);

        // Ray is parallel to the y-axis
        if a < 1e-5 {
            // a is approximately 0
            Vec::new()
        } else {
            let b = 2.0 * object_ray.origin.x() * object_ray.direction.x()
                + 2.0 * object_ray.origin.z() * object_ray.direction.z();
            let c = object_ray.origin.x().powi(2) + object_ray.origin.z().powi(2) - 1.0;

            let discriminant = b.powi(2) - 4.0 * a * c;

            if discriminant < 0.0 {
                Vec::new()
            } else {
                let t0 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
                let t1 = (-b + f64::sqrt(discriminant)) / (2.0 * a);

                vec![t0, t1]
            }
        }
    }

    fn normal_at_local(&self, object_point: &Point3d) -> NormalizedVec3d {
        NormalizedVec3d::new(object_point.x(), 0.0, object_point.z()).unwrap()
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            material: Default::default(),
            transform: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod intersect {
        use crate::math::vector::Vec3d;

        use super::*;

        macro_rules! cylinder_intersect_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (origin, direction, expected) = $value;
                        let cyl: Cylinder = Default::default();
                        let nd = direction.norm().unwrap();
                        let r = Ray::new(origin, nd);

                        let xs = cyl.intersect_local(&r);

                        assert_eq!(xs, expected);
                    }
                )*
            };
        }

        cylinder_intersect_tests! {
            ray_misses_1: (Point3d::new(1.0, 0.0, 0.0), Vec3d::new(0.0, 1.0, 0.0), vec![]),
            ray_misses_2: (Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 1.0, 0.0), vec![]),
            ray_misses_3: (Point3d::new(0.0, 0.0, -5.0), Vec3d::new(1.0, 1.0, 1.0), vec![]),
            ray_hits_1: (Point3d::new(1.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), vec![5.0, 5.0]),
            ray_hits_2: (Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), vec![4.0, 6.0]),
            ray_hits_3: (Point3d::new(0.5, 0.0, -5.0), Vec3d::new(0.1, 1.0, 1.0), vec![6.80798191702732, 7.088723439378861])
        }
    }

    mod normal {
        use super::*;

        macro_rules! cylinder_normal_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (point, expected) = $value;
                        let cyl: Cylinder = Default::default();

                        let n = cyl.normal_at_local(&point);

                        assert_eq!(n, expected);
                    }
                )*
            };
        }

        cylinder_normal_tests! {
            cylinder_normal_pos_x: (Point3d::new(1.0, 0.0, 0.0), NormalizedVec3d::new(1.0, 0.0, 0.0).unwrap()),
            cylinder_normal_neg_z: (Point3d::new(0.0, 5.0, -1.0), NormalizedVec3d::new(0.0, 0.0, -1.0).unwrap()),
            cylinder_normal_pos_z: (Point3d::new(0.0, -2.0, 1.0), NormalizedVec3d::new(0.0, 0.0, 1.0).unwrap()),
            cylinder_normal_neg_x: (Point3d::new(-1.0, 1.0, 0.0), NormalizedVec3d::new(-1.0, 0.0, 0.0).unwrap())
        }
    }
}
