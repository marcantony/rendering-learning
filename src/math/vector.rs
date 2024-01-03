use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::matrix::{Matrix, SquareMatrix};

#[derive(Debug, Clone)]
pub struct Vec3d(Matrix<4, 1>);

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d(Matrix::new([[x], [y], [z], [0.0]]))
    }

    pub fn x(&self) -> f64 {
        self.0.at(0, 0)
    }

    pub fn y(&self) -> f64 {
        self.0.at(1, 0)
    }

    pub fn z(&self) -> f64 {
        self.0.at(2, 0)
    }

    pub fn w(&self) -> f64 {
        self.0.at(3, 0)
    }

    pub fn mag(&self) -> f64 {
        f64::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z())
    }

    pub fn norm(&self) -> Option<Self> {
        let m = self.mag();
        if m == 0.0 {
            None
        } else {
            Some(Vec3d::new(self.x() / m, self.y() / m, self.z() / m))
        }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3d::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn reflect(&self, normal: &Vec3d) -> Self {
        self - &(&(normal * 2.0) * self.dot(normal))
    }
}

impl From<Matrix<4, 1>> for Vec3d {
    fn from(mut value: Matrix<4, 1>) -> Self {
        value.put(3, 0, 0.0);
        Vec3d(value)
    }
}

impl PartialEq for Vec3d {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Add for &Vec3d {
    type Output = Vec3d;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3d::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for &Vec3d {
    type Output = Vec3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Neg for &Vec3d {
    type Output = Vec3d;

    fn neg(self) -> Self::Output {
        Vec3d::new(-self.x(), -self.y(), -self.z())
    }
}

impl Mul<f64> for &Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3d::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Div<f64> for &Vec3d {
    type Output = Vec3d;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3d::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Mul<&Vec3d> for &SquareMatrix<4> {
    type Output = Vec3d;

    fn mul(self, rhs: &Vec3d) -> Self::Output {
        let output_data = self * &rhs.0;
        Vec3d::from(output_data)
    }
}

impl Display for Vec3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3d({}, {}, {})", self.x(), self.y(), self.z())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedVec3d(Vec3d);

impl TryFrom<&Vec3d> for NormalizedVec3d {
    type Error = String;

    fn try_from(value: &Vec3d) -> Result<Self, Self::Error> {
        value
            .norm()
            .map(|n| NormalizedVec3d(n))
            .ok_or(format!("{} cannot be normalized.", value))
    }
}

impl AsRef<Vec3d> for NormalizedVec3d {
    fn as_ref(&self) -> &Vec3d {
        &self.0
    }
}

impl Neg for &NormalizedVec3d {
    type Output = NormalizedVec3d;

    fn neg(self) -> Self::Output {
        NormalizedVec3d::try_from(&-&self.0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_is_tuple_with_w_0() {
        let vector = Vec3d::new(4.3, -4.2, 3.1);

        assert_eq!(vector.x(), 4.3);
        assert_eq!(vector.y(), -4.2);
        assert_eq!(vector.z(), 3.1);
    }

    #[test]
    fn vectors_with_same_vals_are_equal() {
        assert_eq!(Vec3d::new(1.0, 2.0, 3.0), Vec3d::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn vectors_with_different_vals_are_not_equal() {
        let v = Vec3d::new(1.0, 2.0, 3.0);
        assert_ne!(v, Vec3d::new(1.5, 2.0, 3.0));
        assert_ne!(v, Vec3d::new(1.0, 2.5, 3.0));
        assert_ne!(v, Vec3d::new(1.0, 2.0, 3.5));
    }

    #[test]
    fn vectors_with_nan_vals_are_not_equal() {
        assert_ne!(
            Vec3d::new(f64::NAN, 2.0, 3.0),
            Vec3d::new(f64::NAN, 2.0, 3.0)
        );
    }

    mod ops {
        use super::*;

        mod arithmetic {
            use super::*;

            #[test]
            fn add_two_vectors() {
                let a1 = Vec3d::new(3.0, -2.0, 5.0);
                let a2 = Vec3d::new(-2.0, 3.0, 1.0);
                assert_eq!(&a1 + &a2, Vec3d::new(1.0, 1.0, 6.0));
            }

            #[test]
            fn subtract_two_vectors() {
                let v1 = Vec3d::new(3.0, 2.0, 1.0);
                let v2 = Vec3d::new(5.0, 6.0, 7.0);
                assert_eq!(&v1 - &v2, Vec3d::new(-2.0, -4.0, -6.0));
            }

            #[test]
            fn subtract_vector_from_zero_vector() {
                let zero = Vec3d::new(0.0, 0.0, 0.0);
                let v = Vec3d::new(1.0, -2.0, 3.0);
                assert_eq!(&zero - &v, Vec3d::new(-1.0, 2.0, -3.0));
            }

            #[test]
            fn negate_a_vector() {
                let a = Vec3d::new(1.0, -2.0, 3.0);
                assert_eq!(-&a, Vec3d::new(-1.0, 2.0, -3.0));
            }

            #[test]
            fn multiply_vector_by_scalar() {
                let a = Vec3d::new(1.0, -2.0, 3.0);
                assert_eq!(&a * 3.5, Vec3d::new(3.5, -7.0, 10.5));
            }

            #[test]
            fn multiply_vector_by_fraction() {
                let a = Vec3d::new(1.0, -2.0, 3.0);
                assert_eq!(&a * 0.5, Vec3d::new(0.5, -1.0, 1.5));
            }

            #[test]
            fn divide_vector_by_scalar() {
                let a = Vec3d::new(1.0, -2.0, 3.0);
                assert_eq!(&a / 2.0, Vec3d::new(0.5, -1.0, 1.5));
            }
        }

        mod vec_ops {
            use super::*;

            #[test]
            fn compute_magnitude() {
                assert_eq!(Vec3d::new(1.0, 0.0, 0.0).mag(), 1.0);
                assert_eq!(Vec3d::new(0.0, 1.0, 0.0).mag(), 1.0);
                assert_eq!(Vec3d::new(0.0, 0.0, 1.0).mag(), 1.0);
                assert_eq!(Vec3d::new(1.0, 2.0, 3.0).mag(), f64::sqrt(14.0));
                assert_eq!(Vec3d::new(-1.0, -2.0, -3.0).mag(), f64::sqrt(14.0));
            }

            #[test]
            fn normalize_vector() {
                assert_eq!(
                    Vec3d::new(4.0, 0.0, 0.0).norm(),
                    Some(Vec3d::new(1.0, 0.0, 0.0))
                );
                assert_eq!(
                    Vec3d::new(1.0, 2.0, 3.0).norm(),
                    Some(Vec3d::new(
                        1.0 / f64::sqrt(14.0),
                        2.0 / f64::sqrt(14.0),
                        3.0 / f64::sqrt(14.0)
                    ))
                );
                assert_eq!(Vec3d::new(0.0, 0.0, 0.0).norm(), None)
            }

            #[test]
            fn normalized_vector_magnitude() {
                let v = Vec3d::new(1.0, 2.0, 3.0);
                assert_eq!(v.norm().unwrap().mag(), 1.0);
            }

            #[test]
            fn dot_product_of_two_tuples() {
                let a = Vec3d::new(1.0, 2.0, 3.0);
                let b = Vec3d::new(2.0, 3.0, 4.0);
                assert_eq!(a.dot(&b), 20.0);
            }

            #[test]
            fn cross_product_of_two_vectors() {
                let a = Vec3d::new(1.0, 2.0, 3.0);
                let b = Vec3d::new(2.0, 3.0, 4.0);

                assert_eq!(a.cross(&b), Vec3d::new(-1.0, 2.0, -1.0));
                assert_eq!(b.cross(&a), Vec3d::new(1.0, -2.0, 1.0));
            }

            #[test]
            fn reflect_vector_approaching_at_45_degrees() {
                let v = Vec3d::new(1.0, -1.0, 0.0);
                let n = Vec3d::new(0.0, 1.0, 0.0);

                let r = v.reflect(&n);

                assert_eq!(r, Vec3d::new(1.0, 1.0, 0.0));
            }

            #[test]
            fn reflect_vector_off_slanted_surface() {
                let v = Vec3d::new(0.0, -1.0, 0.0);
                let t = std::f64::consts::SQRT_2 / 2.0;
                let n = Vec3d::new(t, t, 0.0);

                let r = v.reflect(&n);

                assert_eq!(r, Vec3d::new(1.0, 0.0, 0.0));
            }
        }
    }

    mod matrix {
        use super::*;

        #[test]
        fn multiply_matrix_by_vector() {
            let a = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            let b = Vec3d::new(1.0, 2.0, 3.0);

            assert_eq!(&a * &b, Vec3d::new(14.0, 22.0, 32.0));
        }
    }

    mod normalized_vector {
        use super::*;

        #[test]
        fn create_normalized_vector() {
            let v = Vec3d::new(1.0, 2.0, 3.0);
            let n = NormalizedVec3d::try_from(&v).unwrap();

            assert_eq!(n.as_ref(), &v.norm().unwrap())
        }

        #[test]
        fn cannot_create_normalized_vector() {
            let v = Vec3d::new(0.0, 0.0, 0.0);
            let n = NormalizedVec3d::try_from(&v);

            assert!(n.is_err())
        }
    }
}
