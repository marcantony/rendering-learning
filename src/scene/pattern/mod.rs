use crate::{
    draw::color::Color,
    math::{matrix::InvertibleMatrix, point::Point3d},
};

use super::object::Object;

pub trait Pattern {
    fn transform(&self) -> &InvertibleMatrix<4>;
    fn at(&self, point: &Point3d) -> Color;

    fn at_object(&self, object: &dyn Object, point: &Point3d) -> Color {
        let object_point = object.transform().inverse() * point;
        self.at(&(self.transform().inverse() * &object_point))
    }
}

pub mod checker3d;
pub mod gradient;
pub mod ring;
pub mod stripe;

#[cfg(test)]
pub mod test_utils {
    use super::*;

    pub struct MockPattern {
        pub transform: InvertibleMatrix<4>,
    }

    impl Pattern for MockPattern {
        fn transform(&self) -> &InvertibleMatrix<4> {
            &self.transform
        }

        fn at(&self, point: &Point3d) -> Color {
            Color::new(point.x(), point.y(), point.z())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::matrix::InvertibleMatrix,
        scene::{object::sphere::Sphere, pattern::test_utils::MockPattern, transformation},
    };

    use super::*;

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let shape = Sphere {
            transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
            ..Default::default()
        };
        let pattern = MockPattern {
            transform: Default::default(),
        };

        let c = pattern.at_object(&shape, &Point3d::new(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let shape = Sphere::unit();
        let pattern = MockPattern {
            transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
        };

        let c = pattern.at_object(&shape, &Point3d::new(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let shape = Sphere {
            transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
            ..Default::default()
        };
        let pattern = MockPattern {
            transform: InvertibleMatrix::try_from(transformation::translation(0.5, 1.0, 1.5))
                .unwrap(),
        };

        let c = pattern.at_object(&shape, &Point3d::new(2.5, 3.0, 3.5));

        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
