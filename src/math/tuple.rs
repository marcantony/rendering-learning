use std::{ops::{Add, Sub, Neg, Mul, Div}, fmt::Display};

#[derive(Debug)]
pub struct Tuple3((f64, f64, f64, f64));

impl Tuple3 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple3((x, y, z, w))
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple3((x, y, z, 1.0))
    }

    pub fn vec(x: f64, y: f64, z: f64) -> Self {
        Tuple3((x, y, z, 0.0))
    }

    pub fn x(&self) -> f64 {
        self.0.0
    }

    pub fn y(&self) -> f64 {
        self.0.1
    }

    pub fn z(&self) -> f64 {
        self.0.2
    }

    pub fn w(&self) -> f64 {
        self.0.3
    }

    pub fn is_point(&self) -> bool {
        self.w() == 1.0
    }

    pub fn is_vec(&self) -> bool {
        self.w() == 0.0
    }

    pub fn mag(&self) -> f64 {
        f64::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w())
    }

    pub fn norm(&self) -> Self {
        let m = self.mag();
        Tuple3::new(self.x() / m, self.y() / m, self.z() / m, self.w() / m)
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() +
        self.y() * rhs.y() +
        self.z() * rhs.z() +
        self.w() * rhs.w()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Tuple3::vec(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x())
    }
}

fn are_equal(a: f64, b: f64) -> bool {
    !a.is_nan() && !b.is_nan() && (a - b).abs() < f64::EPSILON
}

impl PartialEq for Tuple3 {
    fn eq(&self, other: &Self) -> bool {
        are_equal(self.x(), other.x()) &&
        are_equal(self.y(), other.y()) &&
        are_equal(self.z(), other.z()) &&
        are_equal(self.w(), other.w())
    }
}

impl Add for &Tuple3 {
    type Output = Tuple3;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z(), self.w() + rhs.w())
    }
}

impl Sub for &Tuple3 {
    type Output = Tuple3;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z(), self.w() - rhs.w())
    }
}

impl Neg for &Tuple3 {
    type Output = Tuple3;

    fn neg(self) -> Self::Output {
        Tuple3::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl Mul<f64> for &Tuple3 {
    type Output = Tuple3;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs, self.w() * rhs)
    }
}

impl Div<f64> for &Tuple3 {
    type Output = Tuple3;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs, self.w() / rhs)
    }
}

impl Display for Tuple3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x(), self.y(), self.z(), self.w())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_point() {
        let tuple = Tuple3::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(tuple.x(), 4.3);
        assert_eq!(tuple.y(), -4.2);
        assert_eq!(tuple.z(), 3.1);
        assert_eq!(tuple.w(), 1.0);
        assert!(tuple.is_point());
        assert!(!tuple.is_vec());
    }

    #[test]
    fn tuple_with_w_0_is_vec() {
        let tuple = Tuple3::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(tuple.x(), 4.3);
        assert_eq!(tuple.y(), -4.2);
        assert_eq!(tuple.z(), 3.1);
        assert_eq!(tuple.w(), 0.0);
        assert!(!tuple.is_point());
        assert!(tuple.is_vec());
    }

    #[test]
    fn tuples_with_same_vals_are_equal() {
        assert_eq!(Tuple3::new(1.0, 2.0, 3.0, 4.0), Tuple3::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn tuples_with_different_vals_are_not_equal() {
        let t = Tuple3::new(1.0, 2.0, 3.0, 4.0);
        assert_ne!(t, Tuple3::new(1.5, 2.0, 3.0, 4.0));
        assert_ne!(t, Tuple3::new(1.0, 2.5, 3.0, 4.0));
        assert_ne!(t, Tuple3::new(1.0, 2.0, 3.5, 4.0));
        assert_ne!(t, Tuple3::new(1.0, 2.0, 3.0, 4.5));
    }

    #[test]
    fn tuples_with_nan_vals_are_not_equal() {
        assert_ne!(Tuple3::new(f64::NAN, 2.0, 3.0, 4.0), Tuple3::new(f64::NAN, 2.0, 3.0, 4.0));
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = Tuple3::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple3::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vec_creates_tuple_with_w_0() {
        let p = Tuple3::vec(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple3::new(4.0, -4.0, 3.0, 0.0));
    }

    mod ops {
        use super::*;

        mod arithmetic {
            use super::*;

            #[test]
            fn add_two_tuples() {
                let a1 = Tuple3::new(3.0, -2.0, 5.0, 1.0);
                let a2 = Tuple3::new(-2.0, 3.0, 1.0, 0.0);
                assert_eq!(&a1 + &a2, Tuple3::new(1.0, 1.0, 6.0, 1.0));
            }

            #[test]
            fn subtract_two_points() {
                let p1 = Tuple3::point(3.0, 2.0, 1.0);
                let p2 = Tuple3::point(5.0, 6.0, 7.0);
                assert_eq!(&p1 - &p2, Tuple3::vec(-2.0, -4.0, -6.0));
            }

            #[test]
            fn subtract_vector_from_point() {
                let p = Tuple3::point(3.0, 2.0, 1.0);
                let v = Tuple3::vec(5.0, 6.0, 7.0);
                assert_eq!(&p - &v, Tuple3::point(-2.0, -4.0, -6.0));
            }

            #[test]
            fn subtract_two_vectors() {
                let v1 = Tuple3::vec(3.0, 2.0, 1.0);
                let v2 = Tuple3::vec(5.0, 6.0, 7.0);
                assert_eq!(&v1 - &v2, Tuple3::vec(-2.0, -4.0, -6.0));
            }

            #[test]
            fn subtract_vector_from_zero_vector() {
                let zero = Tuple3::vec(0.0, 0.0, 0.0);
                let v = Tuple3::vec(1.0, -2.0, 3.0);
                assert_eq!(&zero - &v, Tuple3::vec(-1.0, 2.0, -3.0));
            }

            #[test]
            fn negate_a_tuple() {
                let a = Tuple3::new(1.0, -2.0, 3.0, -4.0);
                assert_eq!(-&a, Tuple3::new(-1.0, 2.0, -3.0, 4.0));
            }

            #[test]
            fn multiply_tuple_by_scalar() {
                let a = Tuple3::new(1.0, -2.0, 3.0, -4.0);
                assert_eq!(&a * 3.5, Tuple3::new(3.5, -7.0, 10.5, -14.0));
            }

            #[test]
            fn multiply_tuple_by_fraction() {
                let a = Tuple3::new(1.0, -2.0, 3.0, -4.0);
                assert_eq!(&a * 0.5, Tuple3::new(0.5, -1.0, 1.5, -2.0));
            }

            #[test]
            fn divide_tuple_by_scalar() {
                let a = Tuple3::new(1.0, -2.0, 3.0, -4.0);
                assert_eq!(&a / 2.0, Tuple3::new(0.5, -1.0, 1.5, -2.0));
            }
        }

        mod vec_ops {
            use super::*;

            #[test]
            fn compute_magnitude() {
                assert_eq!(Tuple3::vec(1.0, 0.0, 0.0).mag(), 1.0);
                assert_eq!(Tuple3::vec(0.0, 1.0, 0.0).mag(), 1.0);
                assert_eq!(Tuple3::vec(0.0, 0.0, 1.0).mag(), 1.0);
                assert_eq!(Tuple3::vec(1.0, 2.0, 3.0).mag(), f64::sqrt(14.0));
                assert_eq!(Tuple3::vec(-1.0, -2.0, -3.0).mag(), f64::sqrt(14.0));
            }

            #[test]
            fn normalize_vector() {
                assert_eq!(Tuple3::vec(4.0, 0.0, 0.0).norm(), Tuple3::vec(1.0, 0.0, 0.0));
                assert_eq!(Tuple3::vec(1.0, 2.0, 3.0).norm(), Tuple3::vec(1.0 / f64::sqrt(14.0), 2.0 / f64::sqrt(14.0), 3.0 / f64::sqrt(14.0)));
            }

            #[test]
            fn normalized_vector_magnitude() {
                let v = Tuple3::vec(1.0, 2.0, 3.0);
                assert_eq!(v.norm().mag(), 1.0);
            }

            #[test]
            fn dot_product_of_two_tuples() {
                let a = Tuple3::vec(1.0, 2.0, 3.0);
                let b = Tuple3::vec(2.0, 3.0, 4.0);
                assert_eq!(a.dot(&b), 20.0);
            }

            #[test]
            fn cross_product_of_two_vectors() {
                let a = Tuple3::vec(1.0, 2.0, 3.0);
                let b = Tuple3::vec(2.0, 3.0, 4.0);

                assert_eq!(a.cross(&b), Tuple3::vec(-1.0, 2.0, -1.0));
                assert_eq!(b.cross(&a), Tuple3::vec(1.0, -2.0, 1.0));
            }
        }
    }
}
