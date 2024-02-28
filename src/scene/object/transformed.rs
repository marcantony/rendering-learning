use crate::{
    draw::color::Color,
    math::{matrix::InvertibleMatrix, point::Point3d, vector::NormalizedVec3d},
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::Object;

pub struct Transformed<T: Object + ?Sized + 'static> {
    pub child: Box<T>,
    pub transform: InvertibleMatrix<4>,
}

impl<T: Object> Transformed<T> {
    pub fn new(child: T, transform: InvertibleMatrix<4>) -> Self {
        Transformed {
            child: Box::new(child),
            transform,
        }
    }
}

impl<T: Object + ?Sized + 'static> Object for Transformed<T> {
    fn material(&self) -> &Material {
        self.child.material()
    }

    fn color_at(&self, point: &Point3d) -> Color {
        let object_point = self.transform.inverse() * point;
        self.child.color_at(&object_point)
    }

    fn intersect(&self, object_ray: &Ray) -> Vec<Intersection<dyn Object>> {
        let local_ray = object_ray.transform(&self.transform.inverse());
        let xs = self.child.intersect(&local_ray);
        xs.into_iter()
            .map(|x| Intersection::new(x.t(), self as &dyn Object))
            .collect()
    }

    fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d {
        let local_point = self.transform.inverse() * object_point;
        let local_normal = self.child.normal_at(&local_point);
        let world_normal = &self.transform.inverse().transpose() * &*local_normal;
        NormalizedVec3d::try_from(world_normal).unwrap()
    }

    fn bounds(&self) -> super::bounded::Bounds {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::object::test_utils::MockObject;

    use super::*;

    #[test]
    fn material_of_transformed_object_is_material_of_child() {
        let shape = MockObject::default();
        let transformed = Transformed {
            child: Box::new(shape),
            transform: Default::default(),
        };

        assert!(transformed.material() == transformed.child.material());
    }

    mod intersect {
        use crate::{
            math::vector::Vec3d,
            scene::{intersect as is, transformation},
        };

        use super::*;

        #[test]
        fn intersecting_scaled_shape_with_ray() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Transformed {
                child: Box::new(MockObject {
                    intersect_local_arg_expectation: Some(Ray::new(
                        Point3d::new(0.0, 0.0, -2.5),
                        Vec3d::new(0.0, 0.0, 0.5),
                    )),
                    ..Default::default()
                }),
                transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0))
                    .unwrap(),
            };

            s.intersect(&r);
        }

        #[test]
        fn intersecting_translated_shape_with_ray() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = Transformed {
                child: Box::new(MockObject {
                    intersect_local_arg_expectation: Some(Ray::new(
                        Point3d::new(-5.0, 0.0, -5.0),
                        Vec3d::new(0.0, 0.0, 1.0),
                    )),
                    ..Default::default()
                }),
                transform: InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0))
                    .unwrap(),
            };

            s.intersect(&r);
        }

        #[test]
        fn multiple_transformations_apply_in_the_correct_order() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
            let t1 = transformation::translation(5.0, 0.0, 0.0);
            let t2 = transformation::scaling(2.0, 2.0, 2.0);
            let sequenced = transformation::sequence(&vec![t1.clone(), t2.clone()]);

            let expectation = Ray::new(Point3d::new(-5.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 0.5));

            let shape1 = MockObject {
                intersect_local_arg_expectation: Some(expectation.clone()),
                ..Default::default()
            };
            let inner_transformed = Transformed {
                child: Box::new(shape1),
                transform: InvertibleMatrix::try_from(t1).unwrap(),
            };
            let outer_transformed = Transformed {
                child: Box::new(inner_transformed),
                transform: InvertibleMatrix::try_from(t2).unwrap(),
            };

            let transformed_at_once = Transformed {
                child: Box::new(MockObject {
                    intersect_local_arg_expectation: Some(expectation.clone()),
                    ..Default::default()
                }),
                transform: InvertibleMatrix::try_from(sequenced).unwrap(),
            };

            assert_eq!(
                is::test_utils::to_ts(outer_transformed.intersect(&r)),
                is::test_utils::to_ts(transformed_at_once.intersect(&r))
            );
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
            let s = Transformed {
                child: Box::new(MockObject::default()),
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, 1.0, 0.0))
                    .unwrap(),
            };

            let n = s.normal_at(&Point3d::new(0.0, 1.70711, -0.70711));

            vector::test_utils::assert_vec_approx_equals(&n, &Vec3d::new(0.0, 0.70711, -0.70711));
        }

        #[test]
        fn computing_normal_on_transformed_shape() {
            let s = Transformed {
                child: Box::new(MockObject::default()),
                transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
                    transformation::rotation_z(std::f64::consts::PI / 5.0),
                    transformation::scaling(1.0, 0.5, 1.0),
                ]))
                .unwrap(),
            };

            let t = std::f64::consts::SQRT_2 / 2.0;
            let n = s.normal_at(&Point3d::new(0.0, t, -t));

            vector::test_utils::assert_vec_approx_equals(&n, &Vec3d::new(0.0, 0.97014, -0.24254));
        }

        #[test]
        fn multiple_transformations_apply_in_the_correct_order() {
            let p = Point3d::new(1.0, 2.0, 3.0);
            let t1 = transformation::rotation_y(std::f64::consts::FRAC_PI_2);
            let t2 = transformation::scaling(2.0, 2.0, 2.0);
            let sequenced = transformation::sequence(&vec![t1.clone(), t2.clone()]);

            let shape1 = MockObject::default();
            let inner_transformed = Transformed {
                child: Box::new(shape1),
                transform: InvertibleMatrix::try_from(t1).unwrap(),
            };
            let outer_transformed = Transformed {
                child: Box::new(inner_transformed),
                transform: InvertibleMatrix::try_from(t2).unwrap(),
            };

            let transformed_at_once = Transformed {
                child: Box::new(MockObject::default()),
                transform: InvertibleMatrix::try_from(sequenced).unwrap(),
            };

            assert_eq!(
                outer_transformed.normal_at(&p),
                transformed_at_once.normal_at(&p)
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
            let shape = Transformed {
                child: Box::new(MockObject {
                    material: Material {
                        surface: Surface::Pattern(Box::new(pattern)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0))
                    .unwrap(),
            };

            let c = shape.color_at(&Point3d::new(2.0, 3.0, 4.0));

            assert_eq!(c, Color::new(1.0, 1.5, 2.0));
        }

        #[test]
        fn color_at_with_an_object_transformation_and_a_pattern_transformation() {
            let pattern = MockPattern {
                transform: InvertibleMatrix::try_from(transformation::translation(0.5, 1.0, 1.5))
                    .unwrap(),
            };
            let shape = Transformed {
                child: Box::new(MockObject {
                    material: Material {
                        surface: Surface::Pattern(Box::new(pattern)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0))
                    .unwrap(),
            };

            let c = shape.color_at(&Point3d::new(2.5, 3.0, 3.5));

            assert_eq!(c, Color::new(0.75, 0.5, 0.25));
        }

        #[test]
        fn multiple_transformations_apply_in_the_correct_order() {
            let p = Point3d::new(1.0, 2.0, 3.0);
            let t1 = transformation::rotation_y(std::f64::consts::FRAC_PI_2);
            let t2 = transformation::scaling(2.0, 2.0, 2.0);
            let sequenced = transformation::sequence(&vec![t1.clone(), t2.clone()]);

            let shape1 = MockObject {
                material: Material {
                    surface: Surface::Pattern(Box::new(MockPattern {
                        transform: InvertibleMatrix::identity(),
                    })),
                    ..Default::default()
                },
                ..Default::default()
            };
            let inner_transformed = Transformed {
                child: Box::new(shape1),
                transform: InvertibleMatrix::try_from(t1).unwrap(),
            };
            let outer_transformed = Transformed {
                child: Box::new(inner_transformed),
                transform: InvertibleMatrix::try_from(t2).unwrap(),
            };

            let transformed_at_once = Transformed {
                child: Box::new(MockObject {
                    material: Material {
                        surface: Surface::Pattern(Box::new(MockPattern {
                            transform: InvertibleMatrix::identity(),
                        })),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                transform: InvertibleMatrix::try_from(sequenced).unwrap(),
            };

            assert_eq!(
                outer_transformed.color_at(&p),
                transformed_at_once.color_at(&p)
            );
        }
    }
}
