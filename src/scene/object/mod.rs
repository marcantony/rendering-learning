use crate::{
    draw::color::Color,
    math::{matrix::InvertibleMatrix, point::Point3d, vector::NormalizedVec3d},
};

use super::{
    intersect::Intersection,
    material::{self, Material},
    ray::Ray,
};

pub trait Object {
    fn transform(&self) -> &InvertibleMatrix<4>;
    fn transform_by(&mut self, t: &InvertibleMatrix<4>);

    fn material(&self) -> &Material;
    fn color_at(&self, point: &Point3d) -> Color {
        let object_point = self.transform().inverse() * point;
        material::color_at(&self.material().surface, &object_point)
    }

    fn intersect_local(&self, object_ray: &Ray) -> Vec<Intersection<dyn Object>>;
    fn intersect(&self, world_ray: &Ray) -> Vec<Intersection<dyn Object>> {
        let local_ray = world_ray.transform(&self.transform().inverse());
        self.intersect_local(&local_ray)
    }

    fn normal_at_local(&self, object_point: &Point3d) -> NormalizedVec3d;
    fn normal_at(&self, world_point: &Point3d) -> NormalizedVec3d {
        let local_point = self.transform().inverse() * world_point;
        let local_normal = self.normal_at_local(&local_point);
        let world_normal = &self.transform().inverse().transpose() * &*local_normal;
        NormalizedVec3d::try_from(world_normal).unwrap()
    }
}

pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod plane;
pub mod sphere;

#[cfg(test)]
mod tests {
    use super::*;

    struct MockObject {
        transform: InvertibleMatrix<4>,
        intersect_local_arg_expectation: Option<Ray>,
        material: Material,
    }

    impl Object for MockObject {
        fn material(&self) -> &Material {
            &self.material
        }

        fn transform(&self) -> &InvertibleMatrix<4> {
            &self.transform
        }

        fn transform_by(&mut self, _t: &InvertibleMatrix<4>) {
            unimplemented!()
        }

        fn intersect_local(&self, object_ray: &Ray) -> Vec<Intersection<dyn Object>> {
            assert_eq!(
                Some(object_ray),
                self.intersect_local_arg_expectation.as_ref()
            );
            Vec::new()
        }

        fn normal_at_local(&self, object_point: &Point3d) -> NormalizedVec3d {
            NormalizedVec3d::new(object_point.x(), object_point.y(), object_point.z()).unwrap()
        }
    }

    mod intersect {
        use crate::{math::vector::Vec3d, scene::transformation};

        use super::*;

        #[test]
        fn intersecting_scaled_shape_with_ray() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = MockObject {
                intersect_local_arg_expectation: Some(Ray::new(
                    Point3d::new(0.0, 0.0, -2.5),
                    Vec3d::new(0.0, 0.0, 0.5),
                )),
                transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0))
                    .unwrap(),
                material: Default::default(),
            };

            s.intersect(&r);
        }

        #[test]
        fn intersecting_translated_shape_with_ray() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s = MockObject {
                intersect_local_arg_expectation: Some(Ray::new(
                    Point3d::new(-5.0, 0.0, -5.0),
                    Vec3d::new(0.0, 0.0, 1.0),
                )),
                transform: InvertibleMatrix::try_from(transformation::translation(5.0, 0.0, 0.0))
                    .unwrap(),
                material: Default::default(),
            };

            s.intersect(&r);
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
            let s = MockObject {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, 1.0, 0.0))
                    .unwrap(),
                intersect_local_arg_expectation: None,
                material: Default::default(),
            };

            let n = s.normal_at(&Point3d::new(0.0, 1.70711, -0.70711));

            vector::test_utils::assert_vec_approx_equals(&n, &Vec3d::new(0.0, 0.70711, -0.70711));
        }

        #[test]
        fn computing_normal_on_transformed_shape() {
            let s = MockObject {
                transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
                    transformation::rotation_z(std::f64::consts::PI / 5.0),
                    transformation::scaling(1.0, 0.5, 1.0),
                ]))
                .unwrap(),
                intersect_local_arg_expectation: None,
                material: Default::default(),
            };

            let t = std::f64::consts::SQRT_2 / 2.0;
            let n = s.normal_at(&Point3d::new(0.0, t, -t));

            vector::test_utils::assert_vec_approx_equals(&n, &Vec3d::new(0.0, 0.97014, -0.24254));
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
            let shape = MockObject {
                transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0))
                    .unwrap(),
                intersect_local_arg_expectation: None,
                material: Material {
                    surface: Surface::Pattern(Box::new(pattern)),
                    ..Default::default()
                },
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
            let shape = MockObject {
                transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0))
                    .unwrap(),
                intersect_local_arg_expectation: None,
                material: Material {
                    surface: Surface::Pattern(Box::new(pattern)),
                    ..Default::default()
                },
            };

            let c = shape.color_at(&Point3d::new(2.5, 3.0, 3.5));

            assert_eq!(c, Color::new(0.75, 0.5, 0.25));
        }
    }
}
