use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
};

use super::{
    intersect::Intersection,
    material::{self, Material},
    ray::Ray,
};

pub trait Object {
    fn material(&self) -> &Material;
    fn color_at(&self, point: &Point3d) -> Color {
        material::color_at(&self.material().surface, point)
    }
    fn intersect(&self, object_ray: &Ray) -> Vec<Intersection<dyn Object>>;
    fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d;
}

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

    pub struct MockObject {
        pub intersect_local_arg_expectation: Option<Ray>,
        pub material: Material,
    }

    impl Object for MockObject {
        fn material(&self) -> &Material {
            &self.material
        }

        fn intersect(&self, object_ray: &Ray) -> Vec<Intersection<dyn Object>> {
            assert_eq!(
                Some(object_ray),
                self.intersect_local_arg_expectation.as_ref()
            );
            Vec::new()
        }

        fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d {
            NormalizedVec3d::new(object_point.x(), object_point.y(), object_point.z()).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
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
                intersect_local_arg_expectation: None,
                material: Material {
                    surface: Surface::Pattern(Box::new(pattern)),
                    ..Default::default()
                },
            };

            let c = shape.color_at(&Point3d::new(2.0, 3.0, 4.0));

            assert_eq!(c, Color::new(2.0, 3.0, 4.0));
        }
    }
}
