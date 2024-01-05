use crate::{
    math::{matrix::InvertibleMatrix, point::Point3d, vector::NormalizedVec3d},
    scene::{material::Material, ray::Ray},
};

use super::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub transform: InvertibleMatrix<4>,
    pub material: Material,
}

impl Sphere {
    pub fn new(transform: InvertibleMatrix<4>, material: Material) -> Self {
        Sphere {
            transform,
            material,
        }
    }

    pub fn unit() -> Self {
        Sphere::new(InvertibleMatrix::identity(), Default::default())
    }
}

impl Object for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &InvertibleMatrix<4> {
        &self.transform
    }

    fn intersect_local(&self, object_ray: &Ray) -> Option<[f64; 2]> {
        let sphere_to_ray = &object_ray.origin - &Point3d::new(0.0, 0.0, 0.0);

        let a = object_ray.direction.dot(&object_ray.direction);
        let b = 2.0 * object_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let disc_sqrt = f64::sqrt(discriminant);
            let t1 = (-b - disc_sqrt) / (2.0 * a);
            let t2 = (-b + disc_sqrt) / (2.0 * a);

            Some([t1, t2])
        }
    }

    fn normal_at_local(&self, object_point: &Point3d) -> NormalizedVec3d {
        NormalizedVec3d::try_from(object_point - &Point3d::new(0.0, 0.0, 0.0)).unwrap()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::unit()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::vector::Vec3d,
        scene::{ray::Ray, transformation},
    };

    use super::*;

    mod intersection {

        use super::*;

        #[test]
        fn a_ray_intersects_a_sphere_at_two_points() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect_local(&r);

            assert_eq!(xs, Some([4.0, 6.0]));
        }

        #[test]
        fn a_ray_intersects_a_sphere_at_a_tangent() {
            let r = Ray::new(Point3d::new(0.0, 1.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect_local(&r);

            assert_eq!(xs, Some([5.0, 5.0]));
        }

        #[test]
        fn a_ray_misses_a_sphere() {
            let r = Ray::new(Point3d::new(0.0, 2.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect_local(&r);

            assert_eq!(xs, None);
        }

        #[test]
        fn a_ray_originates_inside_a_sphere() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect_local(&r);

            assert_eq!(xs, Some([-1.0, 1.0]));
        }

        #[test]
        fn a_sphere_is_behind_a_ray() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect_local(&r);

            assert_eq!(xs, Some([-6.0, -4.0]));
        }
    }

    mod transform {
        use crate::math::matrix::SquareMatrix;

        use super::*;

        #[test]
        fn a_spheres_default_transformation() {
            let s = Sphere::unit();
            assert_eq!(*s.transform, SquareMatrix::identity());
        }

        #[test]
        fn using_a_different_transformation() {
            let t = transformation::translation(2.0, 3.0, 4.0);
            let s = Sphere {
                transform: InvertibleMatrix::try_from(t.clone()).unwrap(),
                ..Default::default()
            };
            assert_eq!(*s.transform, t);
        }
    }

    mod normal {
        use super::*;

        #[test]
        fn normal_on_a_sphere_at_a_point_on_x_axis() {
            let s = Sphere::unit();

            let n = s.normal_at_local(&Point3d::new(1.0, 0.0, 0.0));

            assert_eq!(*n, Vec3d::new(1.0, 0.0, 0.0));
        }

        #[test]
        fn normal_on_a_sphere_at_a_point_on_y_axis() {
            let s = Sphere::unit();

            let n = s.normal_at_local(&Point3d::new(0.0, 1.0, 0.0));

            assert_eq!(*n, Vec3d::new(0.0, 1.0, 0.0));
        }

        #[test]
        fn normal_on_a_sphere_at_a_point_on_z_axis() {
            let s = Sphere::unit();

            let n = s.normal_at_local(&Point3d::new(0.0, 0.0, 1.0));

            assert_eq!(*n, Vec3d::new(0.0, 0.0, 1.0));
        }

        #[test]
        fn normal_on_a_sphere_at_a_point_at_a_nonaxial_point() {
            let s = Sphere::unit();
            let t = f64::sqrt(3.0) / 3.0;

            let n = s.normal_at_local(&Point3d::new(t, t, t));

            assert_eq!(*n, Vec3d::new(t, t, t));
        }

        #[test]
        fn the_normal_is_a_normalized_vector() {
            let s = Sphere::unit();
            let t = f64::sqrt(3.0) / 3.0;

            let n = s.normal_at_local(&Point3d::new(t, t, t));

            assert_eq!(*n, n.norm().unwrap());
        }
    }

    mod material {
        use super::*;

        #[test]
        fn a_sphere_has_a_default_material() {
            let s: Sphere = Default::default();
            assert_eq!(s.material, Default::default());
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

            assert_eq!(s.material, m);
        }
    }
}
