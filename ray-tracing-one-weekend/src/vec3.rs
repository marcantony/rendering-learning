use std::{
    borrow::BorrowMut,
    ops::{Add, AddAssign, Deref, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

use float_cmp::{assert_approx_eq, ApproxEq, F64Margin};
use rand::Rng;
use rand_distr::{Distribution, UnitSphere};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vec3([f64; 3]);
pub type Point3 = Vec3;

impl Vec3 {
    pub fn zero() -> Self {
        Vec3([0.0, 0.0, 0.0])
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3([x, y, z])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn normalize(&self) -> Self {
        self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let margin = F64Margin::zero().epsilon(1e-8);
        self.x().approx_eq(0.0, margin)
            && self.y().approx_eq(0.0, margin)
            && self.z().approx_eq(0.0, margin)
    }

    pub fn reflect(&self, normal: &NormalizedVec3) -> Self {
        self - 2.0 * self.dot(normal) * (&**normal)
    }

    /// Returns a random vector from the origin to a point on the unit sphere
    pub fn random_unit_vector<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let [x, y, z] = UnitSphere.sample(rng.borrow_mut());
        Vec3::new(x, y, z)
    }

    pub fn random_on_unit_hemisphere<R: Rng + ?Sized>(rng: &mut R, normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector(rng);
        if on_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! vec3_vec3_ops {
    ($($lhs_type:ty, $rhs_type:ty),*) => {
        $(
            impl Add<$rhs_type> for $lhs_type {
                type Output = Vec3;

                fn add(self, rhs: $rhs_type) -> Self::Output {
                    Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
                }
            }

            impl Sub<$rhs_type> for $lhs_type {
                type Output = Vec3;

                fn sub(self, rhs: $rhs_type) -> Self::Output {
                    Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
                }
            }

            impl Mul<$rhs_type> for $lhs_type {
                type Output = Vec3;

                fn mul(self, rhs: $rhs_type) -> Self::Output {
                    Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
                }
            }
        )*
    };
}

vec3_vec3_ops! {
    Vec3, Vec3,
    Vec3, &Vec3,
    &Vec3, Vec3,
    &Vec3, &Vec3
}

macro_rules! vec3_other_ops {
    ($($vec_type:ty),*) => {
        $(
            impl AddAssign<$vec_type> for Vec3 {
                fn add_assign(&mut self, rhs: $vec_type) {
                    self.0[0] += rhs.x();
                    self.0[1] += rhs.y();
                    self.0[2] += rhs.z();
                }
            }

            impl Neg for $vec_type {
                type Output = Vec3;

                fn neg(self) -> Self::Output {
                    Vec3::new(-self.x(), -self.y(), -self.z())
                }
            }

            impl Mul<f64> for $vec_type {
                type Output = Vec3;

                fn mul(self, rhs: f64) -> Self::Output {
                    Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
                }
            }

            impl Mul<$vec_type> for f64 {
                type Output = Vec3;

                fn mul(self, rhs: $vec_type) -> Self::Output {
                    rhs * self
                }
            }

            impl Div<f64> for $vec_type {
                type Output = Vec3;

                fn div(self, rhs: f64) -> Self::Output {
                    self * (1.0 / rhs)
                }
            }
        )*
    };
}

vec3_other_ops! {
    Vec3,
    &Vec3
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedVec3(Vec3);

impl NormalizedVec3 {
    /// Convenience method to create a new normalized vector by hand.
    /// Will panic if the magnitude of the resulting vector is not ~1.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let vec = Vec3::new(x, y, z);
        NormalizedVec3::from_normalized(vec)
    }

    /// Convenience method to create a new normalized vector from a known normalized [Vec3].
    /// Will panic if the magnitude of the input vector is not ~1.
    pub fn from_normalized(vec: Vec3) -> Self {
        assert_approx_eq!(f64, vec.length_squared(), 1.0, epsilon = 1e-5);
        NormalizedVec3(vec)
    }

    pub fn refract(&self, normal: &NormalizedVec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (&**self + cos_theta * &**normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * &**normal;

        r_out_perp + r_out_parallel
    }
}

macro_rules! normalized_tryfrom {
    ($($vec_type:ty),*) => {
        $(
            impl TryFrom<$vec_type> for NormalizedVec3 {
                type Error = &'static str;

                fn try_from(value: $vec_type) -> Result<Self, Self::Error> {
                    let m = value.length_squared();
                    if m.approx_eq(0.0, F64Margin::zero().epsilon(1e-16)) {
                        Err("cannot normalize vector with magnitude 0")
                    } else {
                        Ok(NormalizedVec3(value.normalize()))
                    }
                }
            }
        )*
    };
}

normalized_tryfrom! {
    Vec3,
    &Vec3
}

impl Deref for NormalizedVec3 {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Neg for &NormalizedVec3 {
    type Output = NormalizedVec3;

    fn neg(self) -> Self::Output {
        NormalizedVec3(-&self.0)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::{assert_approx_eq, ApproxEq, F64Margin};

    use super::*;

    impl ApproxEq for &Vec3 {
        type Margin = F64Margin;

        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
            let m = margin.into();
            self.x().approx_eq(other.x(), m)
                && self.y().approx_eq(other.y(), m)
                && self.z().approx_eq(other.z(), m)
        }
    }

    mod vec3 {
        use super::*;

        #[test]
        fn zero_vector() {
            let zeros = Vec3::zero();
            assert_approx_eq!(&Vec3, &zeros, &Vec3::new(0.0, 0.0, 0.0));
        }

        #[test]
        fn vector_accessors() {
            let v = Vec3::new(0.0, 1.0, 2.0);

            assert_eq!(v.x(), 0.0);
            assert_eq!(v.y(), 1.0);
            assert_eq!(v.z(), 2.0);
        }

        #[test]
        fn vector_length() {
            let v = Vec3::new(0.0, 3.0, 4.0);

            assert_eq!(v.length_squared(), 25.0);
            assert_eq!(v.length(), 5.0);
        }

        #[test]
        fn dot_product() {
            let v1 = Vec3::new(2.0, 2.0, 2.0);
            let v2 = Vec3::new(3.0, 0.5, 1.0);

            assert_eq!(v1.dot(&v2), 9.0);
        }

        #[test]
        fn cross_product() {
            let v1 = Vec3::new(1.0, 0.0, 0.0);
            let v2 = Vec3::new(0.0, 2.0, 0.0);

            assert_approx_eq!(&Vec3, &v1.cross(&v2), &Vec3::new(0.0, 0.0, 2.0));
        }

        #[test]
        fn normalize() {
            let v = Vec3::new(2.0, 3.0, 4.0);

            assert_approx_eq!(
                &Vec3,
                &v.normalize(),
                &Vec3::new(0.37139, 0.55708, 0.74278),
                epsilon = 1e-5
            );
        }

        #[test]
        fn vector_indexing() {
            let mut v = Vec3::new(1.0, 2.0, 3.0);
            assert_eq!(v[0], 1.0);
            v[0] = 4.0;
            assert_eq!(v[0], 4.0);
        }

        #[test]
        fn vector_vector_addition() {
            let mut v1 = Vec3::new(1.0, 1.0, 1.0);
            let v2 = Vec3::new(2.0, 2.0, 2.0);
            let expect = Vec3::new(3.0, 3.0, 3.0);

            assert_approx_eq!(&Vec3, &(&v1 + &v2), &expect);
            v1 += &v2;
            assert_approx_eq!(&Vec3, &v1, &expect);
        }

        #[test]
        fn vector_negation() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);

            assert_approx_eq!(&Vec3, &-&v1, &Vec3::new(-1.0, -2.0, -3.0));
        }

        #[test]
        fn vector_subtraction() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = Vec3::new(1.0, 1.0, 1.0);

            assert_approx_eq!(&Vec3, &(&v1 - &v2), &Vec3::new(0.0, 1.0, 2.0));
        }

        #[test]
        fn vector_vector_multiplication() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = Vec3::new(2.0, 0.5, 2.0);

            assert_approx_eq!(&Vec3, &(&v1 * &v2), &Vec3::new(2.0, 1.0, 6.0));
        }

        #[test]
        fn vector_number_multiplication() {
            let mut v1 = Vec3::new(1.0, 2.0, 3.0);
            let t = 2.0;
            let expect = Vec3::new(2.0, 4.0, 6.0);

            assert_approx_eq!(&Vec3, &(&v1 * t), &expect);
            assert_approx_eq!(&Vec3, &(t * &v1), &expect);
            v1 *= t;
            assert_approx_eq!(&Vec3, &v1, &expect);
        }

        #[test]
        fn vector_number_division() {
            let mut v1 = Vec3::new(1.0, 2.0, 3.0);
            let t = 2.0;
            let expect = Vec3::new(0.5, 1.0, 1.5);

            assert_approx_eq!(&Vec3, &(&v1 / t), &expect);
            v1 /= t;
            assert_approx_eq!(&Vec3, &v1, &expect);
        }
    }

    mod normalized_vec3 {
        use super::*;

        #[test]
        fn creating_new_normalized_vector_by_hand() {
            NormalizedVec3::new(0.0, 1.0, 0.0);
        }

        #[test]
        #[should_panic]
        fn trying_to_create_bad_normalized_vector_by_hand() {
            NormalizedVec3::new(2.0, 2.0, 2.0);
        }

        #[test]
        fn creating_normalized_vector_from_vec3() {
            let vec = Vec3::new(3.0, 0.0, 0.0);
            assert_approx_eq!(
                &Vec3,
                &NormalizedVec3::try_from(&vec).unwrap(),
                &Vec3::new(1.0, 0.0, 0.0)
            );
        }

        #[test]
        fn creating_normalized_vector_from_zero_length_vec() {
            let vec = Vec3::new(0.0, 0.0, 0.0);
            assert!(NormalizedVec3::try_from(&vec).is_err());
        }

        #[test]
        fn negating_normalized_vector() {
            let n = NormalizedVec3::new(1.0, 0.0, 0.0);
            assert_approx_eq!(&Vec3, &-&n, &Vec3::new(-1.0, 0.0, 0.0));
        }
    }
}
