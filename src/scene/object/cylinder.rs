use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::{bounded::Bounds, Object, PhysicalObject};

const EPSILON: f64 = 1e-8;

/// A cylinder, by default with radius 1 and infinite length around the y-axis
pub struct Cylinder {
    pub material: Material,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub closed: bool,
}

impl Cylinder {
    fn in_bounds(&self, y: f64) -> bool {
        match (self.minimum, self.maximum) {
            (Some(min), Some(max)) => y > min && y < max,
            (Some(min), None) => y > min,
            (None, Some(max)) => y < max,
            (None, None) => true,
        }
    }

    fn check_cap(&self, ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        x.powi(2) + z.powi(2) <= 1.0
    }

    fn intersects_caps(&self, ray: &Ray) -> Vec<f64> {
        if !self.closed || ray.direction.y().abs() < EPSILON {
            Vec::new()
        } else {
            let mut results = Vec::<f64>::new();

            // check for intersection with lower end cap
            let tl = self
                .minimum
                .map(|min| (min - ray.origin.y()) / ray.direction.y());
            if let Some(val) = tl {
                if self.check_cap(ray, val) {
                    results.push(val);
                }
            }

            let tu = self
                .maximum
                .map(|max| (max - ray.origin.y()) / ray.direction.y());
            if let Some(val) = tu {
                if self.check_cap(ray, val) {
                    results.push(val);
                }
            }

            results
        }
    }
}

impl PhysicalObject for Cylinder {
    fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d {
        let dist2 = object_point.x().powi(2) + object_point.z().powi(2);

        if dist2 < 1.0
            && self
                .maximum
                .map_or(false, |max| object_point.y() >= max - EPSILON)
        {
            NormalizedVec3d::new(0.0, 1.0, 0.0).unwrap()
        } else if dist2 < 1.0
            && self
                .minimum
                .map_or(false, |min| object_point.y() <= min + EPSILON)
        {
            NormalizedVec3d::new(0.0, -1.0, 0.0).unwrap()
        } else {
            NormalizedVec3d::new(object_point.x(), 0.0, object_point.z()).unwrap()
        }
    }
}

impl Object for Cylinder {
    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(
        &self,
        object_ray: &Ray,
    ) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        let a = object_ray.direction.x().powi(2) + object_ray.direction.z().powi(2);

        // Ray is parallel to the y-axis
        let mut wall_xs = if a.abs() < EPSILON {
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
        Bounds {
            minimum: (-1.0, self.minimum.unwrap_or(f64::NEG_INFINITY), -1.0),
            maximum: (1.0, self.maximum.unwrap_or(f64::INFINITY), 1.0),
        }
    }
}

impl Default for Cylinder {
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

        macro_rules! cylinder_intersect_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (origin, direction, expected) = $value;
                        let cyl: Cylinder = Default::default();
                        let nd = direction.norm().unwrap();
                        let r = Ray::new(origin, nd);

                        let xs = is::test_utils::to_ts(&cyl.intersect(&r));

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

        #[test]
        fn intersection_returns_color_and_normal_at_point() {
            let r = Ray::new(Point3d::new(1.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let cylinder = Cylinder::default();

            let xs = cylinder.intersect(&r);

            for x in xs {
                let p = r.position(x.t());
                let n = cylinder.normal_at(&p);
                let c = cylinder.material().surface.color_at(&p);
                assert_eq!(x.normal, n);
                assert_eq!(x.color, c);
            }
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

                        let n = cyl.normal_at(&point);

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

    mod truncate {
        use crate::math::vector::Vec3d;

        use super::*;

        #[test]
        fn the_default_minimum_and_maximum_for_a_cylinder() {
            let cyl: Cylinder = Default::default();

            assert_eq!(cyl.minimum, None);
            assert_eq!(cyl.maximum, None);
        }

        macro_rules! truncated_cylinder_intersect_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (origin, direction, expected) = $value;
                        let cyl = Cylinder {
                            minimum: Some(1.0),
                            maximum: Some(2.0),
                            ..Default::default()
                        };
                        let nd = direction.norm().unwrap();
                        let r = Ray::new(origin, nd);

                        let xs = cyl.intersect(&r);

                        assert_eq!(xs.len(), expected);
                    }
                )*
            };
        }

        truncated_cylinder_intersect_tests! {
            diagonal_ray_from_inside_misses: (Point3d::new(0.0, 1.5, 0.0), Vec3d::new(0.1, 1.0, 0.0), 0),
            ray_above_cylinder_misses: (Point3d::new(0.0, 3.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), 0),
            ray_below_cylinder_misses: (Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), 0),
            ray_on_max_y_misses: (Point3d::new(0.0, 2.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), 0),
            ray_on_min_y_misses: (Point3d::new(0.0, 1.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), 0),
            ray_through_middle_hits: (Point3d::new(0.0, 1.5, -2.0), Vec3d::new(0.0, 0.0, 1.0), 2)
        }
    }

    mod closed {
        use crate::math::vector::Vec3d;

        use super::*;

        #[test]
        fn the_default_closed_value_for_a_cylinder() {
            let cyl: Cylinder = Default::default();

            assert!(!cyl.closed);
        }

        macro_rules! capped_cylinder_intersect_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (origin, direction, expected) = $value;
                        let cyl = Cylinder {
                            minimum: Some(1.0),
                            maximum: Some(2.0),
                            closed: true,
                            ..Default::default()
                        };
                        let nd = direction.norm().unwrap();
                        let r = Ray::new(origin, nd);

                        let xs = cyl.intersect(&r);

                        assert_eq!(xs.len(), expected);
                    }
                )*
            };
        }

        capped_cylinder_intersect_tests! {
            ray_from_above_through_middle: (Point3d::new(0.0, 3.0, 0.0), Vec3d::new(0.0, -1.0, 0.0), 2),
            ray_from_above_diagonally_through_cap_and_side: (Point3d::new(0.0, 3.0, -2.0), Vec3d::new(0.0, -1.0, 2.0), 2),
            ray_from_above_through_cap_and_opposite_corner: (Point3d::new(0.0, 4.0, -2.0), Vec3d::new(0.0, -1.0, 1.0), 2),
            ray_from_below_diagonally_through_cap_and_side: (Point3d::new(0.0, 0.0, -2.0), Vec3d::new(0.0, 1.0, 2.0), 2),
            ray_from_below_through_cap_and_opposite_corner: (Point3d::new(0.0, -1.0, -2.0), Vec3d::new(0.0, 1.0, 1.0), 2)
        }

        macro_rules! capped_cylinder_normal_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (point, expected) = $value;
                        let cyl = Cylinder {
                            minimum: Some(1.0),
                            maximum: Some(2.0),
                            closed: true,
                            ..Default::default()
                        };
                        let n = cyl.normal_at(&point);

                        assert_eq!(n, expected);
                    }
                )*
            };
        }

        capped_cylinder_normal_tests! {
            cylinder_normal_min_cap_1: (Point3d::new(0.0, 1.0, 0.0), NormalizedVec3d::new(0.0, -1.0, 0.0).unwrap()),
            cylinder_normal_min_cap_2: (Point3d::new(0.5, 1.0, 0.0), NormalizedVec3d::new(0.0, -1.0, 0.0).unwrap()),
            cylinder_normal_min_cap_3: (Point3d::new(0.0, 1.0, 0.5), NormalizedVec3d::new(0.0, -1.0, 0.0).unwrap()),
            cylinder_normal_max_cap_1: (Point3d::new(0.0, 2.0, 0.0), NormalizedVec3d::new(0.0, 1.0, 0.0).unwrap()),
            cylinder_normal_max_cap_2: (Point3d::new(0.5, 2.0, 0.0), NormalizedVec3d::new(0.0, 1.0, 0.0).unwrap()),
            cylinder_normal_max_cap_3: (Point3d::new(0.0, 2.0, 0.5), NormalizedVec3d::new(0.0, 1.0, 0.0).unwrap())
        }
    }

    mod bounds {
        use super::*;

        #[test]
        fn bounds_of_an_infinite_cylinder() {
            let cylinder = Cylinder::default();
            assert_eq!(
                cylinder.bounds(),
                Bounds {
                    minimum: (-1.0, f64::NEG_INFINITY, -1.0),
                    maximum: (1.0, f64::INFINITY, 1.0)
                }
            );
        }

        #[test]
        fn bounds_of_a_truncated_cylinder() {
            let cylinder = Cylinder {
                minimum: Some(1.0),
                maximum: Some(3.0),
                ..Default::default()
            };
            assert_eq!(
                cylinder.bounds(),
                Bounds {
                    minimum: (-1.0, 1.0, -1.0),
                    maximum: (1.0, 3.0, 1.0)
                }
            );
        }
    }
}
