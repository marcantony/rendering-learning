use crate::{draw::color::Color, math::point::Point3d};

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Stripe(Color, Color),
}

pub fn at(pattern: &Pattern, point: &Point3d) -> Color {
    match pattern {
        Pattern::Stripe(a, b) => {
            if f64::floor(point.x()) as i64 % 2 == 0 {
                a.clone()
            } else {
                b.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::draw::color;

    use super::*;

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::Stripe(color::white(), color::black());

        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.0, 1.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.0, 2.0, 0.0)), color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::Stripe(color::white(), color::black());

        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 1.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 2.0)), color::white());
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::Stripe(color::white(), color::black());

        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.9, 0.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(1.0, 0.0, 0.0)), color::black());
        assert_eq!(at(&pattern, &Point3d::new(-0.1, 0.0, 0.0)), color::black());
        assert_eq!(at(&pattern, &Point3d::new(-1.0, 0.0, 0.0)), color::black());
        assert_eq!(at(&pattern, &Point3d::new(-1.1, 0.0, 0.0)), color::white());
    }
}
