use crate::{
    draw::color::Color,
    math::{matrix::InvertibleMatrix, point::Point3d, vector::NormalizedVec3d},
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

#[derive(Debug, PartialEq)]
pub enum Object<T: Shape + ?Sized> {
    Shape(Box<T>),
    Group,
    Transformed {
        transform: InvertibleMatrix<4>,
        object: Box<Self>,
    },
    Bounded,
}

impl<T: Shape + ?Sized> Object<T> {
    pub fn material(&self) -> &Material {
        match &self {
            Object::Shape(s) => s.material(),
            Object::Group => todo!(),
            Object::Transformed { object, .. } => object.material(),
            Object::Bounded => todo!(),
        }
    }

    pub fn color_at(&self, point: &Point3d) -> Color {
        match &self {
            Object::Shape(s) => s.color_at(point),
            Object::Group => todo!(),
            Object::Transformed { transform, object } => {
                let transformed_point = transform.inverse() * point;
                object.color_at(&transformed_point)
            }
            Object::Bounded => todo!(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection<Object<T>>> {
        match &self {
            Object::Shape(s) => s
                .intersect(ray)
                .into_iter()
                .map(|i| Intersection::new(i.t(), self))
                .collect(),
            Object::Group => todo!(),
            Object::Transformed { transform, object } => {
                let local_ray = ray.transform(&transform.inverse());
                let xs = object.intersect(&local_ray);
                xs.into_iter()
                    .map(|x| Intersection::new(x.t(), self))
                    .collect()
            }
            Object::Bounded => todo!(),
        }
    }

    pub fn normal_at(&self, point: &Point3d) -> NormalizedVec3d {
        match &self {
            Object::Shape(s) => s.normal_at(point),
            Object::Group => todo!(),
            Object::Transformed { transform, object } => {
                let local_point = transform.inverse() * point;
                let local_normal = object.normal_at(&local_point);
                let world_normal = &transform.inverse().transpose() * &*local_normal;
                NormalizedVec3d::try_from(world_normal).unwrap()
            }
            Object::Bounded => todo!(),
        }
    }

    pub fn bounds(&self) -> Bounds {
        match &self {
            Object::Shape(s) => s.bounds(),
            Object::Group => todo!(),
            Object::Transformed { .. } => todo!(),
            Object::Bounded => todo!(),
        }
    }

    pub fn transform(self, transform: InvertibleMatrix<4>) -> Self {
        Object::Transformed {
            transform,
            object: Box::new(self),
        }
    }
}

impl<T: Shape + 'static> Object<T> {
    pub fn as_dyn(self) -> Object<dyn Shape> {
        match self {
            Object::Shape(s) => Object::Shape(s as Box<dyn Shape>),
            Object::Group => todo!(),
            Object::Transformed { transform, object } => Object::Transformed {
                transform,
                object: Box::new(object.as_dyn()),
            },
            Object::Bounded => todo!(),
        }
    }
}

impl<T: Shape> From<T> for Object<T> {
    fn from(value: T) -> Self {
        Object::Shape(Box::new(value))
    }
}

pub mod bounded;
pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod plane;
pub mod sphere;

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

    mod transformed {
        use crate::scene::object::test_utils::MockObject;

        use super::*;

        #[test]
        fn material_of_transformed_object_is_material_of_child() {
            let shape = MockObject::default();
            let object = Object::from(shape);
            let transformed = object.transform(Default::default());

            if let Object::Transformed { object: o, .. } = &transformed {
                assert!(transformed.material() == o.material());
            } else {
                unreachable!();
            }
        }

        mod intersect {
            use crate::{math::vector::Vec3d, scene::transformation};

            use super::*;

            #[test]
            fn intersecting_scaled_shape_with_ray() {
                let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
                let s = Object::from(MockObject {
                    intersect_local_arg_expectation: Some(Ray::new(
                        Point3d::new(0.0, 0.0, -2.5),
                        Vec3d::new(0.0, 0.0, 0.5),
                    )),
                    ..Default::default()
                })
                .transform(
                    InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
                );

                s.intersect(&r);
            }

            #[test]
            fn intersecting_translated_shape_with_ray() {
                let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
                let s = Object::from(MockObject {
                    intersect_local_arg_expectation: Some(Ray::new(
                        Point3d::new(-5.0, 0.0, -5.0),
                        Vec3d::new(0.0, 0.0, 1.0),
                    )),
                    ..Default::default()
                })
                .transform(
                    InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0)).unwrap(),
                );

                s.intersect(&r);
            }

            #[test]
            fn a_twice_transformed_shape_should_apply_the_inner_transformation_first() {
                let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
                let t1 = Object::from(MockObject {
                    intersect_local_arg_expectation: Some(Ray::new(
                        Point3d::new(-10.0, 0.0, 0.0),
                        Vec3d::new(0.0, 0.0, 0.5),
                    )),
                    ..Default::default()
                })
                .transform(
                    InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0)).unwrap(),
                );
                let t2 = t1.transform(
                    InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
                );

                t2.intersect(&r);
            }
        }

        mod normal_at {
            use crate::{
                math::vector::{self, Vec3d},
                scene::transformation,
            };

            use super::*;

            #[test]
            fn computing_normal_on_translated_shape() {
                let s = Object::from(MockObject::default()).transform(
                    InvertibleMatrix::try_from(transformation::translation(0.0, 1.0, 0.0)).unwrap(),
                );

                let n = s.normal_at(&Point3d::new(0.0, 1.70711, -0.70711));

                vector::test_utils::assert_vec_approx_equals(
                    &n,
                    &Vec3d::new(0.0, 0.70711, -0.70711),
                );
            }

            #[test]
            fn computing_normal_on_transformed_shape() {
                let s = Object::from(MockObject::default()).transform(
                    InvertibleMatrix::try_from(transformation::sequence(&vec![
                        transformation::rotation_z(std::f64::consts::PI / 5.0),
                        transformation::scaling(1.0, 0.5, 1.0),
                    ]))
                    .unwrap(),
                );

                let t = std::f64::consts::SQRT_2 / 2.0;
                let n = s.normal_at(&Point3d::new(0.0, t, -t));

                vector::test_utils::assert_vec_approx_equals(
                    &n,
                    &Vec3d::new(0.0, 0.97014, -0.24254),
                );
            }
        }

        mod color_at {
            use crate::scene::material::Surface;
            use crate::scene::{pattern::test_utils::MockPattern, transformation};

            use super::*;

            #[test]
            fn color_at_with_an_object_transformation() {
                let pattern = MockPattern {
                    transform: Default::default(),
                };
                let shape = Object::from(MockObject {
                    material: Material {
                        surface: Surface::Pattern(Box::new(pattern)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .transform(
                    InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
                );

                let c = shape.color_at(&Point3d::new(2.0, 3.0, 4.0));

                assert_eq!(c, Color::new(1.0, 1.5, 2.0));
            }

            #[test]
            fn color_at_with_an_object_transformation_and_a_pattern_transformation() {
                let pattern = MockPattern {
                    transform: InvertibleMatrix::try_from(transformation::translation(
                        0.5, 1.0, 1.5,
                    ))
                    .unwrap(),
                };
                let shape = Object::from(MockObject {
                    material: Material {
                        surface: Surface::Pattern(Box::new(pattern)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .transform(
                    InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
                );

                let c = shape.color_at(&Point3d::new(2.5, 3.0, 3.5));

                assert_eq!(c, Color::new(0.75, 0.5, 0.25));
            }
        }
    }
}
