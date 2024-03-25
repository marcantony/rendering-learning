use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::{bounded::Bounds, Object, PhysicalObject};

/// A sphere: by default, a unit sphere (of radius 1 and its origin at (0, 0, 0))
pub struct Sphere {
    pub material: Material,
}

impl Sphere {
    pub fn new(material: Material) -> Self {
        Sphere { material }
    }

    pub fn unit() -> Self {
        Sphere::new(Default::default())
    }
}

impl PhysicalObject for Sphere {
    fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d {
        NormalizedVec3d::try_from(object_point - &Point3d::new(0.0, 0.0, 0.0)).unwrap()
    }
}

impl Object for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(
        &self,
        object_ray: &Ray,
    ) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        let sphere_to_ray = &object_ray.origin - &Point3d::new(0.0, 0.0, 0.0);

        let a = object_ray.direction.dot(&object_ray.direction);
        let b = 2.0 * object_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Vec::new()
        } else {
            let disc_sqrt = f64::sqrt(discriminant);
            let t1 = (-b - disc_sqrt) / (2.0 * a);
            let t2 = (-b + disc_sqrt) / (2.0 * a);

            [t1, t2]
                .into_iter()
                .map(|t| super::build_basic_intersection(object_ray, t, self))
                .collect()
        }
    }

    fn bounds(&self) -> Bounds {
        Bounds {
            minimum: Point3d::new(-1.0, -1.0, -1.0),
            maximum: Point3d::new(1.0, 1.0, 1.0),
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::unit()
    }
}

pub fn glass_sphere() -> Sphere {
    Sphere {
        material: Material {
            transparency: 1.0,
            refractive_index: 1.52,
            ..Default::default()
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::vector::Vec3d,
        scene::{intersect as is, ray::Ray},
    };

    use super::*;

    mod intersection {

        use super::*;

        #[test]
        fn a_ray_intersects_a_sphere_at_two_points() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = is::test_utils::to_ts(&s.intersect(&r));

            assert_eq!(xs, vec![4.0, 6.0]);
        }

        #[test]
        fn a_ray_intersects_a_sphere_at_a_tangent() {
            let r = Ray::new(Point3d::new(0.0, 1.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = is::test_utils::to_ts(&s.intersect(&r));

            assert_eq!(xs, vec![5.0, 5.0]);
        }

        #[test]
        fn a_ray_misses_a_sphere() {
            let r = Ray::new(Point3d::new(0.0, 2.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert!(xs.is_empty());
        }

        #[test]
        fn a_ray_originates_inside_a_sphere() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = is::test_utils::to_ts(&s.intersect(&r));

            assert_eq!(xs, vec![-1.0, 1.0]);
        }

        #[test]
        fn a_sphere_is_behind_a_ray() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = is::test_utils::to_ts(&s.intersect(&r));

            assert_eq!(xs, vec![-6.0, -4.0]);
        }

        #[test]
        fn intersection_returns_color_and_normal_at_point() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            for x in xs {
                let p = r.position(x.t());
                let n = s.normal_at(&p);
                let c = s.material().surface.color_at(&p);
                assert_eq!(x.normal, n);
                assert_eq!(x.color, c);
            }
        }
    }

    mod normal {
        use super::*;

        #[test]
        fn normal_on_a_sphere_at_a_point_on_x_axis() {
            let s = Sphere::unit();

            let n = s.normal_at(&Point3d::new(1.0, 0.0, 0.0));

            assert_eq!(*n, Vec3d::new(1.0, 0.0, 0.0));
        }

        #[test]
        fn normal_on_a_sphere_at_a_point_on_y_axis() {
            let s = Sphere::unit();

            let n = s.normal_at(&Point3d::new(0.0, 1.0, 0.0));

            assert_eq!(*n, Vec3d::new(0.0, 1.0, 0.0));
        }

        #[test]
        fn normal_on_a_sphere_at_a_point_on_z_axis() {
            let s = Sphere::unit();

            let n = s.normal_at(&Point3d::new(0.0, 0.0, 1.0));

            assert_eq!(*n, Vec3d::new(0.0, 0.0, 1.0));
        }

        #[test]
        fn normal_on_a_sphere_at_a_point_at_a_nonaxial_point() {
            let s = Sphere::unit();
            let t = f64::sqrt(3.0) / 3.0;

            let n = s.normal_at(&Point3d::new(t, t, t));

            assert_eq!(*n, Vec3d::new(t, t, t));
        }

        #[test]
        fn the_normal_is_a_normalized_vector() {
            let s = Sphere::unit();
            let t = f64::sqrt(3.0) / 3.0;

            let n = s.normal_at(&Point3d::new(t, t, t));

            assert_eq!(*n, n.norm().unwrap());
        }
    }

    mod material {
        use super::*;

        // #[test]
        // fn a_sphere_has_a_default_material() {
        //     let s: Sphere = Default::default();
        //     assert_eq!(s.material, Default::default());
        // }

        #[test]
        fn a_sphere_may_be_assigned_a_material() {
            let m = Material {
                ambient: 1.0,
                ..Default::default()
            };
            let s = Sphere {
                material: m,
                ..Default::default()
            };

            assert_eq!(s.material.ambient, 1.0);
        }
    }
}
