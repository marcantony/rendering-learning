use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone)]
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

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.0[0] += rhs.x();
        self.0[1] += rhs.y();
        self.0[2] += rhs.z();
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
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
