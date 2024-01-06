use crate::{
    draw::color::Color,
    math::{matrix::InvertibleMatrix, point::Point3d},
};

use super::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Stripe(Color, Color, InvertibleMatrix<4>),
}

pub fn at(pattern: &Pattern, point: &Point3d) -> Color {
    match pattern {
        Pattern::Stripe(a, b, _) => {
            if f64::floor(point.x()) as i64 % 2 == 0 {
                a.clone()
            } else {
                b.clone()
            }
        }
    }
}

pub fn at_object(pattern: &Pattern, object: &dyn Object, point: &Point3d) -> Color {
    let object_point = object.transform().inverse() * point;
    match pattern {
        Pattern::Stripe(_, _, t) => at(pattern, &(t.inverse() * &object_point)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        draw::color,
        math::matrix::InvertibleMatrix,
        scene::{object::sphere::Sphere, transformation},
    };

    use super::*;

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::Stripe(color::white(), color::black(), InvertibleMatrix::identity());

        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.0, 1.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.0, 2.0, 0.0)), color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::Stripe(color::white(), color::black(), InvertibleMatrix::identity());

        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 1.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 2.0)), color::white());
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::Stripe(color::white(), color::black(), InvertibleMatrix::identity());

        assert_eq!(at(&pattern, &Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(0.9, 0.0, 0.0)), color::white());
        assert_eq!(at(&pattern, &Point3d::new(1.0, 0.0, 0.0)), color::black());
        assert_eq!(at(&pattern, &Point3d::new(-0.1, 0.0, 0.0)), color::black());
        assert_eq!(at(&pattern, &Point3d::new(-1.0, 0.0, 0.0)), color::black());
        assert_eq!(at(&pattern, &Point3d::new(-1.1, 0.0, 0.0)), color::white());
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Sphere {
            transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
            ..Default::default()
        };
        let pattern = Pattern::Stripe(color::white(), color::black(), InvertibleMatrix::identity());

        let c = at_object(&pattern, &object, &Point3d::new(1.5, 0.0, 0.0));

        assert_eq!(c, color::white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object: Sphere = Default::default();
        let pattern = Pattern::Stripe(
            color::white(),
            color::black(),
            InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
        );

        let c = at_object(&pattern, &object, &Point3d::new(1.5, 0.0, 0.0));

        assert_eq!(c, color::white());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Sphere {
            transform: InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
            ..Default::default()
        };
        let pattern = Pattern::Stripe(
            color::white(),
            color::black(),
            InvertibleMatrix::try_from(transformation::translation(0.5, 0.0, 0.0)).unwrap(),
        );

        let c = at_object(&pattern, &object, &Point3d::new(2.5, 0.0, 0.0));

        assert_eq!(c, color::white());
    }
}
