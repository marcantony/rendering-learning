use crate::{
    draw::color::{self, Color},
    math::matrix::InvertibleMatrix,
};

use super::Pattern;

/// A pattern linearly transitioning from color A to B every 1 unit in the x direction
pub struct Gradient {
    pub a: Color,
    pub b: Color,
    pub transform: InvertibleMatrix<4>,
}

impl Pattern for Gradient {
    fn transform(&self) -> &InvertibleMatrix<4> {
        &self.transform
    }

    fn at_local(&self, point: &crate::math::point::Point3d) -> Color {
        let distance = &self.b - &self.a;
        let fraction = point.x() - f64::floor(point.x());

        &self.a + &(&distance * fraction)
    }
}

impl Default for Gradient {
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
    fn gradient_linearly_interpolates_between_colors() {
        let pattern: Gradient = Default::default();

        assert_eq!(
            pattern.at_local(&Point3d::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_eq!(
            pattern.at_local(&Point3d::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.at_local(&Point3d::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.at_local(&Point3d::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
