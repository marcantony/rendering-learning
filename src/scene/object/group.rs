use crate::{
    math::{matrix::InvertibleMatrix, point::Point3d, vector::NormalizedVec3d},
    scene::{
        intersect::{self, Intersection},
        material::Material,
        ray::Ray,
    },
};

use super::Object;

/// A group of multiple sub-objects
pub struct Group {
    transform: InvertibleMatrix<4>,
    children: Vec<Box<dyn Object>>,
}

impl Group {
    pub fn new(transform: InvertibleMatrix<4>, mut children: Vec<Box<dyn Object>>) -> Group {
        children.iter_mut().for_each(|c| c.transform_by(&transform));
        Group {
            transform,
            children,
        }
    }
}

impl Object for Group {
    fn material(&self) -> &Material {
        unimplemented!()
    }

    fn transform(&self) -> &InvertibleMatrix<4> {
        &self.transform
    }

    fn transform_by(&mut self, t: &InvertibleMatrix<4>) {
        self.transform = t * &self.transform;
        self.children.iter_mut().for_each(|c| c.transform_by(t));
    }

    fn intersect_local(&self, object_ray: &Ray) -> Vec<Intersection<dyn Object>> {
        let mut intersections: Vec<_> = self
            .children
            .iter()
            .flat_map(|obj| obj.intersect(&object_ray))
            .collect();

        intersect::sort(&mut intersections);

        intersections
    }

    fn normal_at_local(&self, _object_point: &Point3d) -> NormalizedVec3d {
        unimplemented!()
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            children: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::object::sphere::Sphere;
    use crate::scene::{intersect as is, transformation};

    use super::*;

    #[test]
    fn creating_a_new_group() {
        let g: Group = Default::default();

        assert_eq!(g.transform, InvertibleMatrix::<4>::identity());
        assert!(g.children.is_empty());
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let g = Group::new(Default::default(), vec![Box::new(Sphere::unit())]);

        assert_eq!(g.children.len(), 1);
    }

    #[test]
    fn adding_a_child_to_a_transformed_group() {
        let s: Sphere = Sphere::unit();
        let g = Group::new(
            InvertibleMatrix::try_from(transformation::translation(1.0, 0.0, 0.0)).unwrap(),
            vec![Box::new(s)],
        );

        assert_eq!(
            **g.children[0].transform(),
            transformation::translation(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn transforming_an_existing_group() {
        let mut g = Group::new(Default::default(), vec![Box::new(Sphere::unit())]);
        g.transform_by(
            &InvertibleMatrix::try_from(transformation::translation(0.0, 2.0, 0.0)).unwrap(),
        );

        assert_eq!(
            **g.children[0].transform(),
            transformation::translation(0.0, 2.0, 0.0)
        );
    }

    mod intersect {
        use crate::{math::vector::Vec3d, scene::transformation};

        use super::*;

        #[test]
        fn intersecting_a_ray_with_an_empty_group() {
            let g: Group = Default::default();
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));

            let xs = g.intersect_local(&r);

            assert!(xs.is_empty());
        }

        #[test]
        fn intersecting_a_ray_with_a_nonempty_group() {
            let s1: Sphere = Default::default();

            let mut s2: Sphere = Default::default();
            s2.transform =
                InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, -3.0)).unwrap();

            let mut s3: Sphere = Default::default();
            s3.transform =
                InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0)).unwrap();

            let g = Group {
                children: vec![Box::new(s1), Box::new(s2), Box::new(s3)],
                ..Default::default()
            };

            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let xs = is::test_utils::to_ts(g.intersect_local(&r));

            assert_eq!(xs, vec![1.0, 3.0, 4.0, 6.0]);
        }

        #[test]
        fn intersecting_a_transformed_group() {
            let mut s: Sphere = Default::default();
            s.transform =
                InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0)).unwrap();

            let g = Group {
                children: vec![Box::new(s)],
                transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0))
                    .unwrap(),
                ..Default::default()
            };

            let r = Ray::new(Point3d::new(10.0, 0.0, -10.0), Vec3d::new(0.0, 0.0, 1.0));
            let xs = g.intersect(&r);

            assert_eq!(xs.len(), 2);
        }
    }

    mod normal {
        use super::*;

        #[test]
        fn finding_the_normal_of_an_object_in_a_group() {
            let g = Group::new(
                InvertibleMatrix::try_from(transformation::translation(1.0, 0.0, 0.0)).unwrap(),
                vec![Box::new(Sphere::unit())],
            );

            let s = g.children[0].as_ref();

            assert_eq!(
                NormalizedVec3d::new(0.0, 1.0, 0.0).unwrap(),
                s.normal_at(&Point3d::new(1.0, 1.0, 0.0))
            );
        }
    }
}
