use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::{bounded::Bounds, Object, PhysicalObject};

#[derive(Default)]
pub struct Cube {
    pub material: Material,
}

impl PhysicalObject for Cube {
    fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d {
        let max_component = object_point
            .x()
            .abs()
            .max(object_point.y().abs())
            .max(object_point.z().abs());

        if max_component == object_point.x().abs() {
            NormalizedVec3d::new(object_point.x(), 0.0, 0.0)
        } else if max_component == object_point.y().abs() {
            NormalizedVec3d::new(0.0, object_point.y(), 0.0)
        } else {
            NormalizedVec3d::new(0.0, 0.0, object_point.z())
        }
        .unwrap()
    }
}

impl Object for Cube {
    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(
        &self,
        object_ray: &Ray,
    ) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        let (xtmin, xtmax) = check_axis(object_ray.origin.x(), object_ray.direction.x());
        let (ytmin, ytmax) = check_axis(object_ray.origin.y(), object_ray.direction.y());
        let (ztmin, ztmax) = check_axis(object_ray.origin.z(), object_ray.direction.z());

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        let ts = if tmin > tmax {
            Vec::new()
        } else {
            vec![tmin, tmax]
        };

        ts.into_iter()
            .map(|t| super::build_basic_intersection(object_ray, t, self))
            .collect()
    }

    fn bounds(&self) -> Bounds {
        Bounds {
            minimum: Point3d::new(-1.0, -1.0, -1.0),
            maximum: Point3d::new(1.0, 1.0, 1.0),
        }
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let tmin = tmin_numerator / direction;
    let tmax = tmax_numerator / direction;

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::vector::Vec3d;
    use crate::scene::intersect as is;

    mod intersect {

        use super::*;

        macro_rules! cube_intersect_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (origin, direction, expected) = $value;

                        let c: Cube = Default::default();
                        let r = Ray::new(origin, direction);

                        let xs = is::test_utils::to_ts(&c.intersect(&r));

                        assert_eq!(xs, expected);
                    }
                )*
            };
        }

        cube_intersect_tests! {
            a_ray_intersects_a_cube_pos_x: (Point3d::new(5.0, 0.5, 0.0), Vec3d::new(-1.0, 0.0, 0.0), vec![4.0, 6.0]),
            a_ray_intersects_a_cube_neg_x: (Point3d::new(-5.0, 0.5, 0.0), Vec3d::new(1.0, 0.0, 0.0), vec![4.0, 6.0]),
            a_ray_intersects_a_cube_pos_y: (Point3d::new(0.5, 5.0, 0.0), Vec3d::new(0.0, -1.0, 0.0), vec![4.0, 6.0]),
            a_ray_intersects_a_cube_neg_y: (Point3d::new(0.5, -5.0, 0.0), Vec3d::new(0.0, 1.0, 0.0), vec![4.0, 6.0]),
            a_ray_intersects_a_cube_pos_z: (Point3d::new(0.5, 0.0, 5.0), Vec3d::new(0.0, 0.0,-1.0), vec![4.0, 6.0]),
            a_ray_intersects_a_cube_neg_z: (Point3d::new(0.5, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0), vec![4.0, 6.0]),
            a_ray_intersects_a_cube_inside: (Point3d::new(0.0, 0.5, 0.0), Vec3d::new(0.0, 0.0, 1.0), vec![-1.0, 1.0])
        }

        cube_intersect_tests! {
            a_ray_misses_a_cube_1: (Point3d::new(-2.0, 0.0, 0.0), Vec3d::new(0.2673, 0.5345, 0.8018), vec![]),
            a_ray_misses_a_cube_2: (Point3d::new(0.0, -2.0, 0.0), Vec3d::new(0.8018, 0.2673, 0.5345), vec![]),
            a_ray_misses_a_cube_3: (Point3d::new(0.0, 0.0, -2.0), Vec3d::new(0.5345, 0.8018, 0.2673), vec![]),
            a_ray_misses_a_cube_4: (Point3d::new(2.0, 0.0, 2.0), Vec3d::new(0.0, 0.0, -1.0), vec![]),
            a_ray_misses_a_cube_5: (Point3d::new(0.0, 2.0, 2.0), Vec3d::new(0.0, -1.0, 0.0), vec![]),
            a_ray_misses_a_cube_6: (Point3d::new(2.0, 2.0, 0.0), Vec3d::new(-1.0, 0.0, 0.0), vec![])
        }

        #[test]
        fn intersection_returns_color_and_normal_at_point() {
            let r = Ray::new(Point3d::new(5.0, 0.5, 0.0), Vec3d::new(-1.0, 0.0, 0.0));
            let cube = Cube::default();

            let xs = cube.intersect(&r);

            for x in xs {
                let p = r.position(x.t());
                let n = cube.normal_at(&p);
                let c = cube.material().surface.color_at(&p);
                assert_eq!(x.normal, n);
                assert_eq!(x.color, c);
            }
        }
    }

    mod normal {
        use super::*;

        macro_rules! cube_normal_tests {
            ($($name:ident: $value:expr),*) => {
                $(
                    #[test]
                    fn $name() {
                        let (point, expected) = $value;

                        let c: Cube = Default::default();

                        let normal = c.normal_at(&point);

                        assert_eq!(*normal, expected)
                    }
                )*
            };
        }

        cube_normal_tests! {
            cube_normal_1: (Point3d::new(1.0, 0.5, -0.8), Vec3d::new(1.0, 0.0, 0.0)),
            cube_normal_2: (Point3d::new(-1.0, -0.2, 0.9), Vec3d::new(-1.0, 0.0, 0.0)),
            cube_normal_3: (Point3d::new(-0.4, 1.0, -0.1), Vec3d::new(0.0, 1.0, 0.0)),
            cube_normal_4: (Point3d::new(0.3, -1.0, -0.7), Vec3d::new(0.0, -1.0, 0.0)),
            cube_normal_5: (Point3d::new(-0.6, 0.3, 1.0), Vec3d::new(0.0, 0.0, 1.0)),
            cube_normal_6: (Point3d::new(0.4, 0.4, -1.0), Vec3d::new(0.0, 0.0, -1.0)),
            cube_normal_7: (Point3d::new(1.0, 1.0, 1.0), Vec3d::new(1.0, 0.0, 0.0)),
            cube_normal_8: (Point3d::new(-1.0, -1.0, -1.0), Vec3d::new(-1.0, 0.0, 0.0))
        }
    }
}
