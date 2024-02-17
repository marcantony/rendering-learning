use crate::{
    draw::color::Color,
    math::{matrix::InvertibleMatrix, point::Point3d},
};

pub trait Pattern {
    fn transform(&self) -> &InvertibleMatrix<4>;
    fn at_local(&self, point: &Point3d) -> Color;
    fn at(&self, point: &Point3d) -> Color {
        self.at_local(&(self.transform().inverse() * point))
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

        fn at_local(&self, point: &Point3d) -> Color {
            Color::new(point.x(), point.y(), point.z())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::matrix::InvertibleMatrix,
        scene::{pattern::test_utils::MockPattern, transformation},
    };

    use super::*;

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let pattern = MockPattern {
            transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
        };

        let c = pattern.at(&Point3d::new(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }
}
