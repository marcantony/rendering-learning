use crate::math::{
    matrix::{Matrix, SquareMatrix},
    point::Point3d,
    vector::{NormalizedVec3d, Vec3d},
};

use super::{intersect::Intersection, material::Material, ray::Ray};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: SquareMatrix<4>,
    pub material: Material,
}

impl Sphere {
    pub fn new(transform: SquareMatrix<4>, material: Material) -> Self {
        Sphere {
            transform,
            material,
        }
    }

    pub fn unit() -> Self {
        Sphere::new(Matrix::identity(), Default::default())
    }

    pub fn transform(&self) -> &SquareMatrix<4> {
        &self.transform
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn intersect(&self, worldspace_ray: &Ray) -> Option<[Intersection; 2]> {
        let r = worldspace_ray.transform(&self.inverse_transform_unchecked());
        let sphere_to_ray = r.origin() - &Point3d::new(0.0, 0.0, 0.0);

        let a = r.direction().dot(r.direction());
        let b = 2.0 * r.direction().dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let disc_sqrt = f64::sqrt(discriminant);
            let t1 = (-b - disc_sqrt) / (2.0 * a);
            let t2 = (-b + disc_sqrt) / (2.0 * a);

            Some([Intersection::new(t1, self), Intersection::new(t2, self)])
        }
    }

    pub fn normal_at(&self, world_point: &Point3d) -> NormalizedVec3d {
        let object_point = &self.inverse_transform_unchecked() * world_point;
        let object_normal = (&object_point - &Point3d::new(0.0, 0.0, 0.0))
            .norm()
            .unwrap();
        let world_normal = &self.inverse_transform_unchecked().transpose() * &object_normal;
        NormalizedVec3d::try_from(Vec3d::new(
            world_normal.x(),
            world_normal.y(),
            world_normal.z(),
        ))
        .unwrap()
    }

    fn inverse_transform_unchecked(&self) -> SquareMatrix<4> {
        self.transform
            .invert()
            .expect("Sphere transform is uninvertible!")
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::unit()
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::{ray::Ray, transformation};

    use super::*;

    mod intersection {
        use super::*;

        #[test]
        fn a_ray_intersects_a_sphere_at_two_points() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(
                xs,
                Some([Intersection::new(4.0, &s), Intersection::new(6.0, &s)])
            );
        }

        #[test]
        fn a_ray_intersects_a_sphere_at_a_tangent() {
            let r = Ray::new(Point3d::new(0.0, 1.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(
                xs,
                Some([Intersection::new(5.0, &s), Intersection::new(5.0, &s)])
            );
        }

        #[test]
        fn a_ray_misses_a_sphere() {
            let r = Ray::new(Point3d::new(0.0, 2.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(xs, None);
        }

        #[test]
        fn a_ray_originates_inside_a_sphere() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(
                xs,
                Some([Intersection::new(-1.0, &s), Intersection::new(1.0, &s)])
            );
        }

        #[test]
        fn a_sphere_is_behind_a_ray() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(
                xs,
                Some([Intersection::new(-6.0, &s), Intersection::new(-4.0, &s)])
            );
        }

        #[test]
        fn intersecting_scaled_sphere_with_ray() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere {
                transform: transformation::scaling(2.0, 2.0, 2.0),
                ..Default::default()
            };

            let xs = s.intersect(&r);

            assert_eq!(
                xs,
                Some([Intersection::new(3.0, &s), Intersection::new(7.0, &s)])
            );
        }

        #[test]
        fn intersecting_translated_sphere_with_ray() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere {
                transform: transformation::translation(5.0, 0.0, 0.0),
                ..Default::default()
            };

            let xs = s.intersect(&r);

            assert_eq!(xs, None);
        }
    }

    mod transform {
        use super::*;

        #[test]
        fn a_spheres_default_transformation() {
            let s = Sphere::unit();
            assert_eq!(s.transform(), &Matrix::identity());
        }

        #[test]
        fn using_a_different_transformation() {
            let t = transformation::translation(2.0, 3.0, 4.0);
            let s = Sphere {
                transform: t.clone(),
                ..Default::default()
            };
            assert_eq!(s.transform(), &t);
        }
    }

    mod normal {
        use super::*;

        #[test]
        fn normal_on_a_sphere_at_a_point_on_x_axis() {
            let s = Sphere::unit();

            let n = s.normal_at(&Point3d::new(1.0, 0.0, 0.0));

            assert_eq!(n.as_ref(), &Vec3d::new(1.0, 0.0, 0.0));
        }

        #[test]
        fn normal_on_a_sphere_at_a_point_on_y_axis() {
            let s = Sphere::unit();

            let n = s.normal_at(&Point3d::new(0.0, 1.0, 0.0));

            assert_eq!(n.as_ref(), &Vec3d::new(0.0, 1.0, 0.0));
        }

        #[test]
        fn normal_on_a_sphere_at_a_point_on_z_axis() {
            let s = Sphere::unit();

            let n = s.normal_at(&Point3d::new(0.0, 0.0, 1.0));

            assert_eq!(n.as_ref(), &Vec3d::new(0.0, 0.0, 1.0));
        }

        #[test]
        fn normal_on_a_sphere_at_a_point_at_a_nonaxial_point() {
            let s = Sphere::unit();
            let t = f64::sqrt(3.0) / 3.0;

            let n = s.normal_at(&Point3d::new(t, t, t));

            assert_eq!(n.as_ref(), &Vec3d::new(t, t, t));
        }

        #[test]
        fn the_normal_is_a_normalized_vector() {
            let s = Sphere::unit();
            let t = f64::sqrt(3.0) / 3.0;

            let n = s.normal_at(&Point3d::new(t, t, t));

            assert_eq!(n.as_ref(), &n.as_ref().norm().unwrap());
        }

        #[test]
        fn computing_the_normal_on_a_translated_sphere() {
            let s = Sphere {
                transform: transformation::translation(0.0, 1.0, 0.0),
                ..Default::default()
            };

            let n = s.normal_at(&Point3d::new(0.0, 1.70711, -0.70711));

            assert_vec_approx_equals(n.as_ref(), &Vec3d::new(0.0, 0.70711, -0.70711));
        }

        #[test]
        fn computing_the_normal_on_a_transformed_sphere() {
            let s = Sphere {
                transform: transformation::sequence(&[
                    transformation::rotation_z(std::f64::consts::PI / 5.0),
                    transformation::scaling(1.0, 0.5, 1.0),
                ]),
                ..Default::default()
            };
            let t = std::f64::consts::SQRT_2 / 2.0;

            let n = s.normal_at(&Point3d::new(0.0, t, -t));

            assert_vec_approx_equals(n.as_ref(), &Vec3d::new(0.0, 0.97014, -0.24254));
        }
    }

    mod material {
        use super::*;

        #[test]
        fn a_sphere_has_a_default_material() {
            let s: Sphere = Default::default();
            assert_eq!(s.material(), &Default::default());
        }

        #[test]
        fn a_sphere_may_be_assigned_a_material() {
            let m = Material {
                ambient: 1.0,
                ..Default::default()
            };
            let s = Sphere {
                material: m.clone(),
                ..Default::default()
            };

            assert_eq!(s.material(), &m);
        }
    }

    fn assert_vec_approx_equals(a: &Vec3d, b: &Vec3d) {
        let tolerance = 1e-5;
        assert!(equal_with_tolerance(a.x(), b.x(), tolerance));
        assert!(equal_with_tolerance(a.y(), b.y(), tolerance));
        assert!(equal_with_tolerance(a.z(), b.z(), tolerance));
        assert!(equal_with_tolerance(a.w(), b.w(), tolerance));
    }

    fn equal_with_tolerance(a: f64, b: f64, tolerance: f64) -> bool {
        f64::abs(a - b) <= tolerance
    }
}
