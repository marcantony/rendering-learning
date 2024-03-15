use crate::{
    draw::color::Color,
    math::{matrix::InvertibleMatrix, vector::NormalizedVec3d},
    scene::{
        intersect::{self, Intersection},
        material::Material,
        ray::Ray,
    },
};

use super::{transformed::Transformed, Object};

/// A group of multiple sub-objects
pub struct Group {
    children: Vec<Box<dyn Object>>,
}

impl Group {
    pub fn new(transform: InvertibleMatrix<4>, children: Vec<Box<dyn Object>>) -> Group {
        let transformed_children = children.into_iter().map(|c| Transformed {
            child: c,
            transform: transform.clone(),
        });
        Group {
            children: transformed_children
                .map(|tc| Box::new(tc) as Box<dyn Object>)
                .collect(),
        }
    }
}

impl Object for Group {
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

    fn bounds(&self) -> super::bounded::Bounds {
        todo!()
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            children: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::intersect as is;
    use crate::scene::object::sphere::Sphere;

    use super::*;

    #[test]
    fn creating_a_new_group() {
        let g: Group = Default::default();

        assert!(g.children.is_empty());
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let g = Group::new(Default::default(), vec![Box::new(Sphere::unit())]);

        assert_eq!(g.children.len(), 1);
    }

    mod intersect {
        use crate::{
            math::{point::Point3d, vector::Vec3d},
            scene::transformation,
        };

        use super::*;

        #[test]
        fn intersecting_a_ray_with_an_empty_group() {
            let g: Group = Default::default();
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));

            let xs = g.intersect(&r);

            assert!(xs.is_empty());
        }

        #[test]
        fn intersecting_a_ray_with_a_nonempty_group() {
            let s1: Sphere = Default::default();

            let s2 = Transformed {
                child: Box::new(Sphere::unit()),
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, -3.0))
                    .unwrap(),
            };

            let s3 = Transformed {
                child: Box::new(Sphere::unit()),
                transform: InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0))
                    .unwrap(),
            };

            let g = Group::new(
                Default::default(),
                vec![Box::new(s1), Box::new(s2), Box::new(s3)],
            );

            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let xs = is::test_utils::to_ts(&g.intersect(&r));

            assert_eq!(xs, vec![1.0, 3.0, 4.0, 6.0]);
        }

        #[test]
        fn intersecting_a_transformed_group() {
            let s = Transformed {
                child: Box::new(Sphere::unit()),
                transform: InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0))
                    .unwrap(),
            };

            let g = Group::new(
                InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
                vec![Box::new(s)],
            );

            let r = Ray::new(Point3d::new(10.0, 0.0, -10.0), Vec3d::new(0.0, 0.0, 1.0));
            let xs = g.intersect(&r);

            assert_eq!(xs.len(), 2);
        }

        #[test]
        fn intersecting_a_group_returns_intersection_of_child() {
            let s1: Sphere = Default::default();
            let g = Group::new(Default::default(), vec![Box::new(s1)]);

            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let group_xs = g.intersect(&r);
            let child_xs = g.children[0].intersect(&r);

            assert!(group_xs == child_xs);
            assert_eq!(group_xs[0].color, child_xs[0].color);
            assert_eq!(group_xs[0].normal, child_xs[0].normal);
        }
    }
}
