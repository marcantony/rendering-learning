use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::{bounded::Bounds, Object, PhysicalObject};

const EPSILON: f64 = 1e-8;

/// A double-napped cone with slope 1 aligned along the y-axis, whose vertex is at the origin
pub struct Cone {
    pub material: Material,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub closed: bool,
}

impl Cone {
    fn in_bounds(&self, y: f64) -> bool {
        match (self.minimum, self.maximum) {
            (Some(min), Some(max)) => y > min && y < max,
            (Some(min), None) => y > min,
            (None, Some(max)) => y < max,
            (None, None) => true,
        }
    }

    fn check_cap(&self, ray: &Ray, y: f64, t: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        x.powi(2) + z.powi(2) <= y.abs()
    }

    fn intersects_caps(&self, ray: &Ray) -> Vec<f64> {
        if !self.closed || ray.direction.y().abs() < EPSILON {
            Vec::new()
        } else {
            let mut results = Vec::<f64>::new();

            // check for intersection with lower end cap
            if let Some(min) = self.minimum {
                let t = (min - ray.origin.y()) / ray.direction.y();
                if self.check_cap(ray, min, t) {
                    results.push(t);
                }
            }

            if let Some(max) = self.maximum {
                let t = (max - ray.origin.y()) / ray.direction.y();
                if self.check_cap(ray, max, t) {
                    results.push(t);
                }
            }

            results
        }
    }
}

impl PhysicalObject for Cone {
    fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d {
        let dist2 = object_point.x().powi(2) + object_point.z().powi(2);

        if self.maximum.map_or(false, |max| {
            dist2 < max.powi(2) && object_point.y() >= max - EPSILON
        }) {
            NormalizedVec3d::new(0.0, 1.0, 0.0).unwrap()
        } else if self.minimum.map_or(false, |min| {
            dist2 < min.powi(2) && object_point.y() <= min + EPSILON
        }) {
            NormalizedVec3d::new(0.0, -1.0, 0.0).unwrap()
        } else {
            let y = f64::sqrt(object_point.x().powi(2) + object_point.z().powi(2));
            let y = if object_point.y() > 0.0 { -y } else { y };
            NormalizedVec3d::new(object_point.x(), y, object_point.z()).unwrap()
        }
    }
}

impl Object for Cone {
    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(
        &self,
        object_ray: &Ray,
    ) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        let a = object_ray.direction.x().powi(2) - object_ray.direction.y().powi(2)
            + object_ray.direction.z().powi(2);
        let b = 2.0 * object_ray.origin.x() * object_ray.direction.x()
            - 2.0 * object_ray.origin.y() * object_ray.direction.y()
            + 2.0 * object_ray.origin.z() * object_ray.direction.z();
        let c = object_ray.origin.x().powi(2) - object_ray.origin.y().powi(2)
            + object_ray.origin.z().powi(2);

        let a_is_zero = a.abs() < EPSILON;
        let b_is_zero = b.abs() < EPSILON;

        let mut wall_xs = if a_is_zero && b_is_zero {
            Vec::new()
        } else if a_is_zero && !b_is_zero {
            let t = -c / (2.0 * b);
            vec![t]
        } else {
            let discriminant = b.powi(2) - 4.0 * a * c;

            if discriminant < 0.0 {
                Vec::new()
            } else {
                let t0 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
                let t1 = (-b + f64::sqrt(discriminant)) / (2.0 * a);

                let mut results = Vec::<f64>::new();
                let y0 = object_ray.origin.y() + t0 * object_ray.direction.y();
                if self.in_bounds(y0) {
                    results.push(t0)
                }
                let y1 = object_ray.origin.y() + t1 * object_ray.direction.y();
                if self.in_bounds(y1) {
                    results.push(t1)
                }

                results
            }
        };

        let mut cap_xs = self.intersects_caps(object_ray);

        wall_xs.append(&mut cap_xs);

        wall_xs
            .into_iter()
            .map(|t| super::build_basic_intersection(object_ray, t, self))
            .collect()
    }

    fn bounds(&self) -> Bounds {
        let y_min = self.minimum.unwrap_or(f64::NEG_INFINITY);
        let y_max = self.maximum.unwrap_or(f64::INFINITY);

        // Slope is 1, so the radius at y_min/y_max is equal to y_min/y_max
        let radius = y_max.abs().max(y_min.abs());
        Bounds {
            minimum: [-radius, y_min, -radius],
            maximum: [radius, y_max, radius],
        }
    }
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            material: Default::default(),
            minimum: None,
            maximum: None,
            closed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::intersect as is;

    use super::*;

    mod intersect {
        use crate::math::vector::Vec3d;

        use super::*;

        macro_rules! cone_intersect_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (origin, direction, expected) = $value;
                        let cone: Cone = Default::default();
                        let nd = direction.norm().unwrap();
                        let r = Ray::new(origin, nd);

                        let xs: Vec<f64> = is::test_utils::to_ts(&cone.intersect(&r));

                        assert_eq!(xs, expected);
                    }
                )*
            };
        }

        cone_intersect_tests! {
            intersect_1: (Point3d::new(0.0, 1e-6, -5.0), Vec3d::new(0.0, 0.0, 1.0), vec![4.999999000844085, 5.000000999155915]),
            intersect_2: (Point3d::new(0.0, 0.0, -5.0), Vec3d::new(1.0, 1.0, 1.0), vec![8.660254037844386, 8.660254037844386]),
            intersect_3: (Point3d::new(1.0, 1.0, -5.0), Vec3d::new(-0.5, -1.0, 1.0), vec![4.550055679356349, 49.449944320643645])
        }

        #[test]
        fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
            let shape: Cone = Default::default();
            let nd = Vec3d::new(0.0, 1.0, 1.0).norm().unwrap();
            let r = Ray::new(Point3d::new(0.0, 0.0, -1.0), nd);

            let xs = is::test_utils::to_ts(&shape.intersect(&r));

            assert_eq!(xs, vec![0.3535533905932738]);
        }

        macro_rules! cone_endcap_intersect_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (origin, direction, expected) = $value;
                        let cone = Cone {
                            minimum: Some(-0.5),
                            maximum: Some(0.5),
                            closed: true,
                            ..Default::default()
                        };
                        let nd = direction.norm().unwrap();
                        let r = Ray::new(origin, nd);

                        let xs = cone.intersect(&r);

                        assert_eq!(xs.len(), expected);
                    }
                )*
            };
        }

        cone_endcap_intersect_tests! {
            endcap_intersect_1: (Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 1.0, 0.0), 0),
            endcap_intersect_2: (Point3d::new(0.0, 0.0, -0.25), Vec3d::new(0.0, 1.0, 1.0), 2),
            endcap_intersect_3: (Point3d::new(0.0, 0.0, -0.25), Vec3d::new(0.0, 1.0, 0.0), 4)
        }

        #[test]
        fn intersection_returns_color_and_normal_at_point() {
            let r = Ray::new(Point3d::new(0.0, 1.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let cone = Cone::default();

            let xs = cone.intersect(&r);

            for x in xs {
                let p = r.position(x.t());
                let n = cone.normal_at(&p);
                let c = cone.material().surface.color_at(&p);
                assert_eq!(x.normal, n);
                assert_eq!(x.color, c);
            }
        }
    }

    mod normal {
        use super::*;

        macro_rules! cone_normal_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (point, expected) = $value;
                        let cone: Cone = Default::default();

                        let n = cone.normal_at(&point);

                        assert_eq!(n, expected);
                    }
                )*
            };
        }

        cone_normal_tests! {
            normal_1: (Point3d::new(1.0, 1.0, 1.0), NormalizedVec3d::new(1.0, -std::f64::consts::SQRT_2, 1.0).unwrap()),
            normal_2: (Point3d::new(-1.0, -1.0, 0.0), NormalizedVec3d::new(-1.0, 1.0, 0.0).unwrap())
        }

        #[test]
        #[should_panic]
        fn no_normal_vector_at_vertex() {
            let cone: Cone = Default::default();

            cone.normal_at(&Point3d::new(0.0, 0.0, 0.0));
        }
    }

    mod bounds {
        use super::*;

        #[test]
        fn bounds_of_an_infinite_cone() {
            let cone = Cone::default();
            assert_eq!(
                cone.bounds(),
                Bounds {
                    minimum: [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY],
                    maximum: [f64::INFINITY, f64::INFINITY, f64::INFINITY],
                }
            );
        }

        #[test]
        fn bounds_of_a_truncated_cone() {
            let cone = Cone {
                minimum: Some(-1.0),
                maximum: Some(3.0),
                ..Default::default()
            };
            assert_eq!(
                cone.bounds(),
                Bounds {
                    minimum: [-3.0, -1.0, -3.0],
                    maximum: [3.0, 3.0, 3.0]
                }
            );
        }
    }
}
