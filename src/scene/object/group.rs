use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
    scene::{
        intersect::{self, Intersection},
        material::Material,
        ray::Ray,
    },
};

use super::{bounded::Bounds, Object};

/// A group of multiple sub-objects
pub struct Group<T> {
    children: Vec<T>,
}

impl<T: Object> Group<T> {
    pub fn new(children: Vec<T>) -> Self {
        Group { children }
    }
}

impl<T: Object> Object for Group<T> {
    fn material(&self) -> &Material {
        unimplemented!()
    }

    fn intersect(
        &self,
        object_ray: &Ray,
    ) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        let mut intersections: Vec<_> = self
            .children
            .iter()
            .flat_map(|obj| obj.intersect(&object_ray))
            .collect();

        intersect::sort(&mut intersections);

        intersections
    }

    fn bounds(&self) -> Bounds {
        let points: Vec<_> = self
            .children
            .iter()
            .flat_map(|c| [c.bounds().minimum, c.bounds().maximum].into_iter())
            .collect();
        Bounds::from_points(&points).unwrap_or(Bounds {
            minimum: Point3d::new(0.0, 0.0, 0.0),
            maximum: Point3d::new(0.0, 0.0, 0.0),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::intersect as is;
    use crate::scene::object::sphere::Sphere;

    use super::*;

    #[test]
    fn adding_a_child_to_a_group() {
        let g = Group::new(vec![Sphere::unit()]);

        assert_eq!(g.children.len(), 1);
    }

    mod intersect {
        use crate::{
            math::{matrix::InvertibleMatrix, point::Point3d, vector::Vec3d},
            scene::{object::transformed::Transformed, transformation},
        };

        use super::*;

        #[test]
        fn intersecting_a_ray_with_an_empty_group() {
            let g = Group::<Sphere>::new(vec![]);
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));

            let xs = g.intersect(&r);

            assert!(xs.is_empty());
        }

        #[test]
        fn intersecting_a_ray_with_a_nonempty_group() {
            let s1: Sphere = Default::default();

            let s2 = Transformed::new(
                Sphere::unit(),
                InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, -3.0)).unwrap(),
            );

            let s3 = Transformed::new(
                Sphere::unit(),
                InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0)).unwrap(),
            );

            let g: Group<Box<dyn Object>> =
                Group::new(vec![Box::new(s1), Box::new(s2), Box::new(s3)]);

            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let xs = is::test_utils::to_ts(&g.intersect(&r));

            assert_eq!(xs, vec![1.0, 3.0, 4.0, 6.0]);
        }

        #[test]
        fn intersecting_a_transformed_group() {
            let s = Transformed::new(
                Sphere::unit(),
                InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0)).unwrap(),
            );

            let g = Transformed::new(
                Group::new(vec![s]),
                InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
            );

            let r = Ray::new(Point3d::new(10.0, 0.0, -10.0), Vec3d::new(0.0, 0.0, 1.0));
            let xs = g.intersect(&r);

            assert_eq!(xs.len(), 2);
        }

        #[test]
        fn intersecting_a_group_returns_intersection_of_child() {
            let s1: Sphere = Default::default();
            let g = Group::new(vec![s1]);

            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let group_xs = g.intersect(&r);
            let child_xs = g.children[0].intersect(&r);

            assert!(group_xs == child_xs);
            assert_eq!(group_xs[0].color, child_xs[0].color);
            assert_eq!(group_xs[0].normal, child_xs[0].normal);
        }
    }

    mod bounds {
        use crate::{math::point::Point3d, scene::object::test_utils::MockObject};

        use super::*;

        #[test]
        fn bounds_of_a_group_covers_bounds_of_all_children() {
            let c1 = MockObject {
                bounds: Bounds {
                    minimum: Point3d::new(0.0, 0.0, 0.0),
                    maximum: Point3d::new(2.0, 1.0, 1.0),
                },
                ..Default::default()
            };
            let c2 = MockObject {
                bounds: Bounds {
                    minimum: Point3d::new(-1.0, -1.0, 0.0),
                    maximum: Point3d::new(1.0, 1.0, 1.0),
                },
                ..Default::default()
            };
            let c3 = MockObject {
                bounds: Bounds {
                    minimum: Point3d::new(0.5, 0.5, 0.5),
                    maximum: Point3d::new(1.0, 1.0, 10.0),
                },
                ..Default::default()
            };

            let group = Group::new(vec![c1, c2, c3]);

            assert_eq!(
                group.bounds(),
                Bounds {
                    minimum: Point3d::new(-1.0, -1.0, 0.0),
                    maximum: Point3d::new(2.0, 1.0, 10.0),
                }
            )
        }

        #[test]
        fn bounds_of_an_empty_group_are_minimal() {
            let g = Group::<Sphere>::new(vec![]);

            assert_eq!(
                g.bounds(),
                Bounds {
                    minimum: Point3d::new(0.0, 0.0, 0.0),
                    maximum: Point3d::new(0.0, 0.0, 0.0),
                }
            )
        }
    }
}
