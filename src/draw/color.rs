use std::ops::{Add, Mul, Sub};

use crate::math::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct Color(Matrix<3, 1>);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color(Matrix::new([[r], [g], [b]]))
    }

    pub fn r(&self) -> f64 {
        self.0.at(0, 0)
    }

    pub fn g(&self) -> f64 {
        self.0.at(1, 0)
    }

    pub fn b(&self) -> f64 {
        self.0.at(2, 0)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Add for &Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color(&self.0 + &rhs.0)
    }
}

impl Sub for &Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color(&self.0 - &rhs.0)
    }
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(&self.0 * rhs)
    }
}

impl Mul for &Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
    }
}

pub fn black() -> Color {
    Color::new(0.0, 0.0, 0.0)
}

pub fn white() -> Color {
    Color::new(1.0, 1.0, 1.0)
}

pub fn red() -> Color {
    Color::new(1.0, 0.0, 0.0)
}

pub fn green() -> Color {
    Color::new(0.0, 1.0, 0.0)
}

pub fn blue() -> Color {
    Color::new(0.0, 0.0, 1.0)
}

#[cfg(test)]
pub mod test_utils {
    use crate::math::util;

    use super::*;

    pub fn assert_colors_approx_equal(a: &Color, b: &Color) {
        assert!(util::test_utils::are_within_tolerance(a.r(), b.r(), 1e-5));
        assert!(util::test_utils::are_within_tolerance(a.g(), b.g(), 1e-5));
        assert!(util::test_utils::are_within_tolerance(a.b(), b.b(), 1e-5));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.r(), -0.5);
        assert_eq!(c.g(), 0.4);
        assert_eq!(c.b(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(&c1 + &c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(&c1 - &c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(&c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(&c1 * &c2, Color::new(0.9, 0.2, 0.04));
    }
}
