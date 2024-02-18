use crate::{
    math::{point::Point3d, vector::NormalizedVec3d},
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::Object;

type Bound = (f64, f64, f64);
#[derive(Debug, PartialEq, Clone)]
pub struct Bounds {
    pub minimum: Bound,
    pub maximum: Bound,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            minimum: (f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            maximum: (f64::INFINITY, f64::INFINITY, f64::INFINITY),
        }
    }
}

pub struct Bounded<T: Object> {
    bounds: Bounds,
    child: T,
}

impl<T: Object> Bounded<T> {
    pub fn new(child: T) -> Self {
        Bounded {
            bounds: child.bounds(),
            child,
        }
    }

    fn test(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) = check_axis(
            self.bounds.minimum.0,
            self.bounds.maximum.0,
            ray.origin.x(),
            ray.direction.x(),
        );
        let (ytmin, ytmax) = check_axis(
            self.bounds.minimum.1,
            self.bounds.maximum.1,
            ray.origin.y(),
            ray.direction.y(),
        );
        let (ztmin, ztmax) = check_axis(
            self.bounds.minimum.2,
            self.bounds.maximum.2,
            ray.origin.z(),
            ray.direction.z(),
        );

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        tmin <= tmax
    }
}

fn check_axis(min: f64, max: f64, origin: f64, speed: f64) -> (f64, f64) {
    let distance_to_min = min - origin;
    let distance_to_max = max - origin;

    let tmin = distance_to_min / speed;
    let tmax = distance_to_max / speed;

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl<T: Object> Object for Bounded<T> {
    fn material(&self) -> &Material {
        self.child.material()
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection<dyn Object>> {
        if self.test(ray) {
            self.child.intersect(ray)
        } else {
            Vec::new()
        }
    }

    fn normal_at(&self, point: &Point3d) -> NormalizedVec3d {
        self.child.normal_at(point)
    }

    fn bounds(&self) -> Bounds {
        self.bounds.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::vector::Vec3d, scene::object::test_utils::MockObject};

    use super::*;

    #[test]
    fn normal_of_a_bounded_object_is_normal_of_child() {
        let shape = MockObject::default();
        let bounded = Bounded::new(shape);

        assert_eq!(
            bounded.normal_at(&Point3d::new(1.0, 2.0, 3.0)),
            NormalizedVec3d::new(1.0, 2.0, 3.0).unwrap()
        )
    }

    #[test]
    fn bounds_of_bounded_object_are_bounds_of_child() {
        let shape = MockObject::default();
        let shape_bounds = shape.bounds();
        let bounded = Bounded::new(shape);

        assert_eq!(shape_bounds, bounded.bounds());
    }

    #[test]
    fn return_child_intersection_when_boundary_test_passes() {
        let ray = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(1.0, 2.0, 3.0));
        let shape = MockObject {
            intersect_local_arg_expectation: Some(ray.clone()),
            ..Default::default()
        };
        let bounded = Bounded::new(shape);

        assert_eq!(bounded.intersect(&ray).len(), 1);
    }

    #[test]
    fn material_of_bounded_object_is_material_of_child() {
        let shape = MockObject::default();
        let bounded = Bounded::new(shape);

        assert!(bounded.material() == bounded.child.material());
    }

    mod boundary_test {
        use super::*;

        macro_rules! boundary_intersect_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (ray, expected) = $value;

                        let shape = MockObject {
                            bounds: Bounds {
                                minimum: (2.0, 2.0, 2.0),
                                maximum: (4.0, 4.0, 4.0),
                            },
                            ..Default::default()
                        };
                        let bounded = Bounded::new(shape);

                        assert_eq!(bounded.test(&ray), expected);
                    }
                )*
            };
        }

        boundary_intersect_tests! {
            a_ray_intersects_pos_x: (Ray::new(Point3d::new(5.0, 3.0, 3.0), Vec3d::new(-1.0, 0.0, 0.0)), true),
            a_ray_intersects_neg_x: (Ray::new(Point3d::new(-5.0, 3.0, 3.0), Vec3d::new(1.0, 0.0, 0.0)), true),
            a_ray_intersects_pos_y: (Ray::new(Point3d::new(3.0, 5.0, 3.0), Vec3d::new(0.0, -1.0, 0.0)), true),
            a_ray_intersects_neg_y: (Ray::new(Point3d::new(3.0, -5.0, 3.0), Vec3d::new(0.0, 1.0, 0.0)), true),
            a_ray_intersects_pos_z: (Ray::new(Point3d::new(3.0, 3.0, 5.0), Vec3d::new(0.0, 0.0,-1.0)), true),
            a_ray_intersects_neg_z: (Ray::new(Point3d::new(3.0, 3.0, -5.0), Vec3d::new(0.0, 0.0, 1.0)), true),
            a_ray_intersects_inside: (Ray::new(Point3d::new(3.0, 3.0, 3.0), Vec3d::new(0.0, 0.0, 1.0)), true),
            a_ray_hits_the_edge: (Ray::new(Point3d::new(5.0, 4.0, 4.0), Vec3d::new(-1.0, 0.0, 0.0)), true)
        }

        boundary_intersect_tests! {
            a_ray_misses_pos_x_1: (Ray::new(Point3d::new(5.0, 4.1, 3.0), Vec3d::new(-1.0, 0.0, 0.0)), false),
            a_ray_misses_pos_x_2: (Ray::new(Point3d::new(5.0, 1.9, 3.0), Vec3d::new(-1.0, 0.0, 0.0)), false),
            a_ray_misses_pos_x_3: (Ray::new(Point3d::new(5.0, 3.0, 4.1), Vec3d::new(-1.0, 0.0, 0.0)), false),
            a_ray_misses_pos_x_4: (Ray::new(Point3d::new(5.0, 3.0, 1.9), Vec3d::new(-1.0, 0.0, 0.0)), false),
            a_ray_misses_neg_x_1: (Ray::new(Point3d::new(-5.0, 4.1, 3.0), Vec3d::new(1.0, 0.0, 0.0)), false),
            a_ray_misses_neg_x_2: (Ray::new(Point3d::new(-5.0, 1.9, 3.0), Vec3d::new(1.0, 0.0, 0.0)), false),
            a_ray_misses_neg_x_3: (Ray::new(Point3d::new(-5.0, 3.0, 4.1), Vec3d::new(1.0, 0.0, 0.0)), false),
            a_ray_misses_neg_x_4: (Ray::new(Point3d::new(-5.0, 3.0, 1.9), Vec3d::new(1.0, 0.0, 0.0)), false),

            a_ray_misses_pos_y_1: (Ray::new(Point3d::new(4.1, 5.0, 3.0), Vec3d::new(0.0, -1.0, 0.0)), false),
            a_ray_misses_pos_y_2: (Ray::new(Point3d::new(1.9, 5.0, 3.0), Vec3d::new(0.0, -1.0, 0.0)), false),
            a_ray_misses_pos_y_3: (Ray::new(Point3d::new(3.0, 5.0, 4.1), Vec3d::new(0.0, -1.0, 0.0)), false),
            a_ray_misses_pos_y_4: (Ray::new(Point3d::new(3.0, 5.0, 1.9), Vec3d::new(0.0, -1.0, 0.0)), false),
            a_ray_misses_neg_y_1: (Ray::new(Point3d::new(4.1, -5.0, 3.0), Vec3d::new(0.0, 1.0, 0.0)), false),
            a_ray_misses_neg_y_2: (Ray::new(Point3d::new(1.9, -5.0, 3.0), Vec3d::new(0.0, 1.0, 0.0)), false),
            a_ray_misses_neg_y_3: (Ray::new(Point3d::new(3.0, -5.0, 4.1), Vec3d::new(0.0, 1.0, 0.0)), false),
            a_ray_misses_neg_y_4: (Ray::new(Point3d::new(3.0, -5.0, 1.9), Vec3d::new(0.0, 1.0, 0.0)), false),

            a_ray_misses_pos_z_1: (Ray::new(Point3d::new(4.1, 3.0, 5.0), Vec3d::new(0.0, 0.0, -1.0)), false),
            a_ray_misses_pos_z_2: (Ray::new(Point3d::new(1.9, 3.0, 5.0), Vec3d::new(0.0, 0.0, -1.0)), false),
            a_ray_misses_pos_z_3: (Ray::new(Point3d::new(3.0, 4.1, 5.0), Vec3d::new(0.0, 0.0, -1.0)), false),
            a_ray_misses_pos_z_4: (Ray::new(Point3d::new(3.0, 1.9, 5.0), Vec3d::new(0.0, 0.0, -1.0)), false),
            a_ray_misses_neg_z_1: (Ray::new(Point3d::new(4.1, 3.0, -5.0), Vec3d::new(0.0, 0.0, 1.0)), false),
            a_ray_misses_neg_z_2: (Ray::new(Point3d::new(1.9, 3.0, -5.0), Vec3d::new(0.0, 0.0, 1.0)), false),
            a_ray_misses_neg_z_3: (Ray::new(Point3d::new(3.0, 4.1, -5.0), Vec3d::new(0.0, 0.0, 1.0)), false),
            a_ray_misses_neg_z_4: (Ray::new(Point3d::new(3.0, 1.9, -5.0), Vec3d::new(0.0, 0.0, 1.0)), false)
        }

        #[test]
        fn a_ray_intersects_a_bounding_box_going_to_infinity() {
            let shape = MockObject {
                bounds: Bounds {
                    minimum: (-1.0, f64::NEG_INFINITY, -1.0),
                    maximum: (1.0, f64::INFINITY, 1.0),
                },
                ..Default::default()
            };
            let bounded = Bounded::new(shape);
            let ray = Ray::new(Point3d::new(5.0, 1e100, 0.0), Vec3d::new(-1.0, 0.0, 0.0));

            assert!(bounded.test(&ray))
        }
    }
}
