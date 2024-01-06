use crate::{
    draw::color::{self, Color},
    math::{matrix::InvertibleMatrix, point::Point3d},
};

use super::Pattern;

/// A pattern alternating between colors A and B in 1-unit rings from the origin on the xz plane
pub struct Ring {
    pub a: Color,
    pub b: Color,
    pub transform: InvertibleMatrix<4>,
}

impl Pattern for Ring {
    fn transform(&self) -> &InvertibleMatrix<4> {
        &self.transform
    }

    fn at(&self, point: &Point3d) -> Color {
        let radius = f64::sqrt(point.x() * point.x() + point.z() * point.z());

        if f64::floor(radius) as i64 % 2 == 0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
}

impl Default for Ring {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern: Ring = Default::default();

        assert_eq!(pattern.at(&Point3d::new(0.0, 0.0, 0.0)), color::white());
        assert_eq!(pattern.at(&Point3d::new(1.0, 0.0, 0.0)), color::black());
        assert_eq!(pattern.at(&Point3d::new(0.0, 0.0, 1.0)), color::black());
        // 0.708 = just slightly more than √2/2
        assert_eq!(pattern.at(&Point3d::new(0.708, 0.0, 0.708)), color::black());
    }
}
