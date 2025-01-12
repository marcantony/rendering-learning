use std::ops::Mul;

use crate::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct Matrix3(pub [[f64; 3]; 3]);

impl Matrix3 {
    pub fn transpose(&self) -> Self {
        let mut out = [[0.0; 3]; 3];

        for n in 0..3 {
            for m in 0..3 {
                out[n][m] = self.0[m][n];
            }
        }

        Matrix3(out)
    }
}

impl Mul<&Matrix3> for &Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: &Matrix3) -> Self::Output {
        let mut out = [[0.0; 3]; 3];

        for n in 0..3 {
            for m in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += self.0[n][k] * rhs.0[k][m];
                }
                out[n][m] = sum;
            }
        }

        Matrix3(out)
    }
}

impl Mul<&Vec3> for &Matrix3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        let mut out = [0.0; 3];

        let vec_data = [rhs.x(), rhs.y(), rhs.z()];

        for n in 0..3 {
            let mut sum = 0.0;
            for k in 0..3 {
                sum += self.0[n][k] * vec_data[k];
            }
            out[n] = sum;
        }

        Vec3::new(out[0], out[1], out[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_two_matrices() {
        let a = Matrix3([[1.0, 2.0, -1.0], [3.0, 2.0, 0.0], [-4.0, 0.0, 2.0]]);

        let b = Matrix3([[3.0, 4.0, 2.0], [0.0, 1.0, 0.0], [-2.0, 0.0, 1.0]]);

        assert_eq!(
            &a * &b,
            Matrix3([[5.0, 6.0, 1.0], [9.0, 14.0, 6.0], [-16.0, -16.0, -6.0]])
        );
    }

    #[test]
    fn multiply_matrix_and_vec() {
        let a = Matrix3([[1.0, 2.0, -1.0], [3.0, 2.0, 0.0], [-4.0, 0.0, 2.0]]);

        let b = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(&a * &b, Vec3::new(2.0, 7.0, 2.0));
    }

    #[test]
    fn transpose_matrix() {
        let a = Matrix3([[1.0; 3], [2.0; 3], [3.0; 3]]);

        assert_eq!(a.transpose(), Matrix3([[1.0, 2.0, 3.0]; 3]));
    }
}
