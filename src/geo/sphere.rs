use crate::math::{tuple::Tuple3, matrix::Matrix};

use super::{ray::Ray, intersect::Intersection};

#[derive(Debug)]
pub struct Sphere {
    transform: Matrix<4>
}

impl Sphere {
    pub fn new(transform: Matrix<4>) -> Self {
        Sphere { transform }
    }

    pub fn unit() -> Self {
        Sphere::new(Matrix::identity())
    }

    pub fn transform(&self) -> &Matrix<4> {
        &self.transform
    }

    pub fn intersect(&self, worldspace_ray: &Ray) -> Option<[Intersection; 2]> {
        let r = worldspace_ray.transform(
            &self.transform().invert().expect("Sphere transform is uninvertible!"));
        let sphere_to_ray = r.origin() - &Tuple3::point(0.0, 0.0, 0.0);

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
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::{transformation, tuple::Tuple3}, geo::ray::Ray};

    use super::*;

    mod intersection {
        use super::*;

        #[test]
        fn a_ray_intersects_a_sphere_at_two_points() {
            let r = Ray::new(Tuple3::point(0.0, 0.0, -5.0), Tuple3::vec(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(xs, Some([Intersection::new(4.0, &s), Intersection::new(6.0, &s)]));
        }

        #[test]
        fn a_ray_intersects_a_sphere_at_a_tangent() {
            let r = Ray::new(Tuple3::point(0.0, 1.0, -5.0), Tuple3::vec(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(xs, Some([Intersection::new(5.0, &s), Intersection::new(5.0, &s)]));
        }

        #[test]
        fn a_ray_misses_a_sphere() {
            let r = Ray::new(Tuple3::point(0.0, 2.0, -5.0), Tuple3::vec(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(xs, None);
        }

        #[test]
        fn a_ray_originates_inside_a_sphere() {
            let r = Ray::new(Tuple3::point(0.0, 0.0, 0.0), Tuple3::vec(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(xs, Some([Intersection::new(-1.0, &s), Intersection::new(1.0, &s)]));
        }

        #[test]
        fn a_sphere_is_behind_a_ray() {
            let r = Ray::new(Tuple3::point(0.0, 0.0, 5.0), Tuple3::vec(0.0, 0.0, 1.0));
            let s = Sphere::unit();

            let xs = s.intersect(&r);

            assert_eq!(xs, Some([Intersection::new(-6.0, &s), Intersection::new(-4.0, &s)]));
        }

        #[test]
        fn intersecting_scaled_sphere_with_ray() {
            let r = Ray::new(Tuple3::point(0.0, 0.0, -5.0), Tuple3::vec(0.0, 0.0, 1.0));
            let s = Sphere::new(transformation::scaling(2.0, 2.0, 2.0));

            let xs = s.intersect(&r);

            assert_eq!(xs, Some([Intersection::new(3.0, &s), Intersection::new(7.0, &s)]));
        }

        #[test]
        fn intersecting_translated_sphere_with_ray() {
            let r = Ray::new(Tuple3::point(0.0, 0.0, -5.0), Tuple3::vec(0.0, 0.0, 1.0));
            let s = Sphere::new(transformation::translation(5.0, 0.0, 0.0));

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
            let s = Sphere::new(t.clone());
            assert_eq!(s.transform(), &t);
        }
    }
}
