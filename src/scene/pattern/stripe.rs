use crate::{
    draw::color::{self, Color},
    math::{matrix::InvertibleMatrix, point::Point3d},
};

use super::Pattern;

/// A pattern alternating between colors A and B every 1 unit in the x direction
#[derive(Debug, Clone, PartialEq)]
pub struct Stripe {
    pub a: Color,
    pub b: Color,
    pub transform: InvertibleMatrix<4>,
}

impl Pattern for Stripe {
    fn transform(&self) -> &InvertibleMatrix<4> {
        &self.transform
    }

    fn at(&self, point: &Point3d) -> Color {
        if f64::floor(point.x()) as i64 % 2 == 0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

impl Default for Stripe {
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
    use crate::draw::color;

    use super::*;

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern: Stripe = Default::default();

        assert_eq!(pattern.at(&Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(pattern.at(&Point3d::new(0.0, 1.0, 0.0)), color::white());
        assert_eq!(pattern.at(&Point3d::new(0.0, 2.0, 0.0)), color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern: Stripe = Default::default();

        assert_eq!(pattern.at(&Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(pattern.at(&Point3d::new(0.0, 0.0, 1.0)), color::white());
        assert_eq!(pattern.at(&Point3d::new(0.0, 0.0, 2.0)), color::white());
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern: Stripe = Default::default();

        assert_eq!(pattern.at(&Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(pattern.at(&Point3d::new(0.9, 0.0, 0.0)), color::white());
        assert_eq!(pattern.at(&Point3d::new(1.0, 0.0, 0.0)), color::black());
        assert_eq!(pattern.at(&Point3d::new(-0.1, 0.0, 0.0)), color::black());
        assert_eq!(pattern.at(&Point3d::new(-1.0, 0.0, 0.0)), color::black());
        assert_eq!(pattern.at(&Point3d::new(-1.1, 0.0, 0.0)), color::white());
    }
}
