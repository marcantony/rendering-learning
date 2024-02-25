use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
};

use self::bounded::Bounds;

use super::{
    intersect::Intersection,
    material::{self, Material},
    ray::Ray,
};

pub trait Shape {
    fn material(&self) -> &Material;
    fn color_at(&self, point: &Point3d) -> Color {
        material::color_at(&self.material().surface, point)
    }
    fn intersect(&self, object_ray: &Ray) -> Vec<Intersection<dyn Shape>>;
    fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d;
    fn bounds(&self) -> Bounds;
}

pub enum Object<T: Shape + ?Sized> {
    Shape(Box<T>),
    Group,
    Transformed,
    Bounded,
}

impl<T: Shape + ?Sized> Object<T> {
    pub fn material(&self) -> &Material {
        match &self {
            Object::Shape(s) => s.material(),
            Object::Group => todo!(),
            Object::Transformed => todo!(),
            Object::Bounded => todo!(),
        }
    }

    pub fn color_at(&self, point: &Point3d) -> Color {
        match &self {
            Object::Shape(s) => s.color_at(point),
            Object::Group => todo!(),
            Object::Transformed => todo!(),
            Object::Bounded => todo!(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection<dyn Shape>> {
        match &self {
            Object::Shape(s) => s.intersect(ray),
            Object::Group => todo!(),
            Object::Transformed => todo!(),
            Object::Bounded => todo!(),
        }
    }

    pub fn normal_at(&self, point: &Point3d) -> NormalizedVec3d {
        match &self {
            Object::Shape(s) => s.normal_at(point),
            Object::Group => todo!(),
            Object::Transformed => todo!(),
            Object::Bounded => todo!(),
        }
    }

    pub fn bounds(&self) -> Bounds {
        match &self {
            Object::Shape(s) => s.bounds(),
            Object::Group => todo!(),
            Object::Transformed => todo!(),
            Object::Bounded => todo!(),
        }
    }
}

pub mod bounded;
pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod plane;
pub mod sphere;
pub mod transformed;

#[cfg(test)]
pub mod test_utils {
    use super::*;

    #[derive(Default)]
    pub struct MockObject {
        pub intersect_local_arg_expectation: Option<Ray>,
        pub material: Material,
        pub bounds: Bounds,
    }

    impl Shape for MockObject {
        fn material(&self) -> &Material {
            &self.material
        }

        fn intersect(&self, object_ray: &Ray) -> Vec<Intersection<dyn Shape>> {
            if let Some(expected) = self.intersect_local_arg_expectation.as_ref() {
                assert_eq!(object_ray, expected);
            }
            vec![Intersection::new(1.0, self as &dyn Shape)]
        }

        fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d {
            NormalizedVec3d::new(object_point.x(), object_point.y(), object_point.z()).unwrap()
        }

        fn bounds(&self) -> Bounds {
            self.bounds.clone()
        }
    }
}

#[cfg(test)]
mod shape_tests {
    use super::test_utils::MockObject;

    use super::*;

    mod color_at {
        use crate::scene::material::Surface;
        use crate::scene::pattern::test_utils::MockPattern;

        use super::*;

        #[test]
        fn color_at_untransformed() {
            let pattern = MockPattern {
                transform: Default::default(),
            };
            let shape = MockObject {
                material: Material {
                    surface: Surface::Pattern(Box::new(pattern)),
                    ..Default::default()
                },
                ..Default::default()
            };

            let c = shape.color_at(&Point3d::new(2.0, 3.0, 4.0));

            assert_eq!(c, Color::new(2.0, 3.0, 4.0));
        }
    }
}

#[cfg(test)]
mod object_tests {
    use super::*;

    mod shape {
        use crate::math::vector::Vec3d;
        use crate::scene::intersect::test_utils as is;

        use self::test_utils::MockObject;

        use super::*;

        #[test]
        fn material_returns_material_of_underlying_shape() {
            let object = Object::Shape(Box::new(MockObject::default()));

            if let Object::Shape(s) = &object {
                assert!(s.material() == object.material());
            } else {
                panic!("broke the test");
            };
        }

        #[test]
        fn color_at_returns_color_of_underlying_shape() {
            let object = Object::Shape(Box::new(MockObject::default()));

            if let Object::Shape(s) = &object {
                let point = Point3d::new(1.0, 2.0, 3.0);
                assert_eq!(s.color_at(&point), object.color_at(&point));
            } else {
                panic!("broke the test");
            };
        }

        #[test]
        fn intersect_returns_intersect_of_underlying_shape() {
            let object = Object::Shape(Box::new(MockObject::default()));

            if let Object::Shape(s) = &object {
                let ray = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(1.0, 0.0, 0.0));
                assert_eq!(
                    is::to_ts(s.intersect(&ray)),
                    is::to_ts(object.intersect(&ray))
                );
            } else {
                panic!("broke the test");
            };
        }

        #[test]
        fn normal_at_returns_normal_of_underlying_shape() {
            let object = Object::Shape(Box::new(MockObject::default()));

            if let Object::Shape(s) = &object {
                let point = Point3d::new(1.0, 0.0, 0.0);
                assert_eq!(s.normal_at(&point), object.normal_at(&point));
            } else {
                panic!("broke the test");
            };
        }

        #[test]
        fn bounds_returns_bounds_of_underlying_shape() {
            let object = Object::Shape(Box::new(MockObject::default()));

            if let Object::Shape(s) = &object {
                assert!(s.bounds() == object.bounds());
            } else {
                panic!("broke the test");
            };
        }
    }
}
