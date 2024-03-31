use crate::{
    draw::color::{self, Color},
    math::matrix::InvertibleMatrix,
};

use super::Pattern;

pub struct Checker3d {
    pub a: Color,
    pub b: Color,
    pub transform: InvertibleMatrix<4>,
}

impl Pattern for Checker3d {
    fn transform(&self) -> &InvertibleMatrix<4> {
        &self.transform
    }

    fn at_local(&self, point: &crate::math::point::Point3d) -> Color {
        if (f64::floor(point.x()) + f64::floor(point.y()) + f64::floor(point.z())) as i64 % 2 == 0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

impl Default for Checker3d {
    fn default() -> Self {
        Self {
            a: color::white(),
            b: color::black(),
            transform: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::point::Point3d;

    use super::*;

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern: Checker3d = Default::default();

        assert_eq!(
            pattern.at_local(&Point3d::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_eq!(
            pattern.at_local(&Point3d::new(0.99, 0.0, 0.0)),
            color::white()
        );
        assert_eq!(
            pattern.at_local(&Point3d::new(1.01, 0.0, 0.0)),
            color::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern: Checker3d = Default::default();

        assert_eq!(
            pattern.at_local(&Point3d::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_eq!(
            pattern.at_local(&Point3d::new(0.0, 0.99, 0.0)),
            color::white()
        );
        assert_eq!(
            pattern.at_local(&Point3d::new(0.0, 1.01, 0.0)),
            color::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern: Checker3d = Default::default();

        assert_eq!(
            pattern.at_local(&Point3d::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_eq!(
            pattern.at_local(&Point3d::new(0.0, 0.0, 0.99)),
            color::white()
        );
        assert_eq!(
            pattern.at_local(&Point3d::new(0.0, 0.0, 1.01)),
            color::black()
        );
    }
}
