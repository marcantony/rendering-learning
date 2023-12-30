use std::ops::Mul;

use super::{util, tuple::Tuple3};

#[derive(Debug)]
pub struct Matrix<const N: usize>([[f64; N]; N]);

impl<const N: usize> Matrix<N> {
    pub fn new(data: [[f64; N]; N]) -> Self {
        Matrix(data)
    }

    pub fn identity() -> Self {
        let mut data = [[0.0; N]; N];

        for i in 0..N {
            data[i][i] = 1.0;
        }

        Matrix::new(data)
    }

    pub fn at(&self, x: usize, y: usize) -> f64 {
        self.0[x][y]
    }

    pub fn transpose(&self) -> Self {
        let mut data = [[0.0; N]; N];

        for n in 0..N {
            for m in 0..N {
                data[m][n] = self.0[n][m];
            }
        }

        Matrix::new(data)
    }
}

pub trait Determinant {
    fn determinant(&self) -> f64;
}
pub trait Submatrix {
    type Output;

    fn submatrix(&self, n: usize, m: usize) -> Self::Output;
}
pub trait Cofactor: Submatrix where Self::Output: Determinant {
    fn minor(&self, n: usize, m: usize) -> f64 {
        self.submatrix(n, m).determinant()
    }

    fn cofactor(&self, n: usize, m: usize) -> f64 {
        let minor = self.minor(n, m);
        if (n + m) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}
pub trait Invertible where Self: Sized {
    fn is_invertible(&self) -> bool;
    fn invert(&self) -> Option<Self>;
}

impl<const N: usize> Cofactor for Matrix<N> where Self: Submatrix, Self::Output: Determinant {}
impl<const N: usize> Determinant for Matrix<N> where Self: Cofactor, <Self as Submatrix>::Output: Determinant {
    fn determinant(&self) -> f64 {
        let mut sum = 0.0;

        for i in 0..N {
            sum += self.0[0][i] * self.cofactor(0, i);
        }

        sum
    }
}
impl<const N: usize> Invertible for Matrix<N> where Self: Determinant + Cofactor, <Self as Submatrix>::Output: Determinant {
    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn invert(&self) -> Option<Self> {
        let det = self.determinant();

        if det == 0.0 {
            None
        } else {
            let mut data = [[0.0; N]; N];

            for n in 0..N {
                for m in 0..N {
                    let c = self.cofactor(n, m);
                    data[m][n] = c / det;
                }
            }

            Some(Matrix::new(data))
        }
    }
}

impl Determinant for Matrix<2> {
    fn determinant(&self) -> f64 {
        self.0[0][0] * self.0[1][1] - self.0[0][1] * self.0[1][0]
    }
}

impl Submatrix for Matrix<3> {
    type Output = Matrix<2>;

    fn submatrix(&self, n: usize, m: usize) -> Self::Output {
        let mut data = [[0.0; 2]; 2];

        for i in 0..3 {
            if i == n {
                continue;
            }
            for j in 0..3 {
                if j == m {
                    continue;
                }

                let new_i = if i >= n {
                    i - 1
                } else {
                    i
                };
                let new_j = if j >= m {
                    j - 1
                } else {
                    j
                };

                data[new_i][new_j] = self.0[i][j];
            }
        }

        Matrix::new(data)
    }
}

impl Submatrix for Matrix<4> {
    type Output = Matrix<3>;

    fn submatrix(&self, n: usize, m: usize) -> Self::Output {
        let mut data = [[0.0; 3]; 3];

        for i in 0..4 {
            if i == n {
                continue;
            }
            for j in 0..4 {
                if j == m {
                    continue;
                }

                let new_i = if i >= n {
                    i - 1
                } else {
                    i
                };
                let new_j = if j >= m {
                    j - 1
                } else {
                    j
                };

                data[new_i][new_j] = self.0[i][j];
            }
        }

        Matrix::new(data)
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, other: &Self) -> bool {
        let vals = |m: [[f64; N]; N]| m.into_iter().flat_map(|row| row.into_iter()).collect();
        let lhs_vals: Vec<f64> = vals(self.0);
        let rhs_vals: Vec<f64> = vals(other.0);

        assert_eq!(lhs_vals.len(), rhs_vals.len(), "Both matrices of size {} don't have the same number of elements.", N);

        lhs_vals.into_iter().zip(rhs_vals.into_iter()).all(|(x, y)| util::are_equal(x, y))
    }
}

impl<const N: usize> Mul for &Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = [[0.0; N]; N];

        for n in 0..N {
            for m in 0..N {
                let mut sum = 0.0;
                for i in 0..N {
                    sum += self.0[n][i] * rhs.0[i][m];
                }
                output[n][m] = sum;
            }
        }

        Matrix::new(output)
    }
}

impl Mul<&Tuple3> for &Matrix<4> {
    type Output = Tuple3;

    fn mul(self, rhs: &Tuple3) -> Self::Output {
        const N: usize = 4;

        let mut output = [0.0; N];

        for n in 0..N {
            output[n] =
                self.0[n][0] * rhs.x() +
                self.0[n][1] * rhs.y() +
                self.0[n][2] * rhs.z() +
                self.0[n][3] * rhs.w();
        }

        Tuple3::new(output[0], output[1], output[2], output[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_and_inspect_4x4_matrix() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]
        ]);

        assert_eq!(m.at(0, 0), 1.0);
        assert_eq!(m.at(0, 3), 4.0);
        assert_eq!(m.at(1, 0), 5.5);
        assert_eq!(m.at(1, 2), 7.5);
        assert_eq!(m.at(2, 2), 11.0);
        assert_eq!(m.at(3, 0), 13.5);
        assert_eq!(m.at(3, 2), 15.5);
    }

    #[test]
    fn a_2x2_matrix_is_representable() {
        let m = Matrix::new([
            [-3.0, 5.0],
            [1.0, -2.0]
        ]);

        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(0, 1), 5.0);
        assert_eq!(m.at(1, 0), 1.0);
        assert_eq!(m.at(1, 1), -2.0);
    }

    #[test]
    fn a_3x3_matrix_is_representable() {
        let m = Matrix::new([
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]
        ]);

        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(1, 1), -2.0);
        assert_eq!(m.at(2, 2), 1.0);
    }

    mod equality {
        use super::*;

        #[test]
        fn equality_with_identical_matrices() {
            let a = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0]
            ]);

            let b = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0]
            ]);

            assert_eq!(a, b);
        }

        #[test]
        fn equality_with_different_matrices() {
            let a = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0]
            ]);

            let b = Matrix::new([
                [2.0, 3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0, 9.0],
                [8.0, 7.0, 6.0, 5.0],
                [4.0, 3.0, 2.0, 1.0]
            ]);

            assert_ne!(a, b);
        }
    }

    mod multiply {
        use crate::math::tuple::Tuple3;

        use super::*;

        #[test]
        fn multiply_two_matrices() {
            let a = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0]
            ]);

            let b = Matrix::new([
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0]
            ]);

            assert_eq!(&a * &b, Matrix::new([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0]
            ]));
        }

        #[test]
        fn multiply_matrix_by_tuple() {
            let a = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0]
            ]);

            let b = Tuple3::new(1.0, 2.0, 3.0, 1.0);

            assert_eq!(&a * &b, Tuple3::new(18.0, 24.0, 33.0, 1.0));
        }

        #[test]
        fn multiply_matrix_by_identity() {
            let a = Matrix::new([
                [0.0, 1.0, 2.0, 4.0],
                [1.0, 2.0, 4.0, 8.0],
                [2.0, 4.0, 8.0, 16.0],
                [4.0, 8.0, 16.0, 32.0]
            ]);

            let b = Matrix::identity();

            assert_eq!(&a * &b, a);
        }
    }

    mod transpose {
        use super::*;

        #[test]
        fn transpose_a_matrix() {
            let a = Matrix::new([
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0]
            ]);

            assert_eq!(a.transpose(), Matrix::new([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0]
            ]))
        }

        #[test]
        fn transpose_identity_matrix() {
            let a = Matrix::<4>::identity();

            assert_eq!(a.transpose(), a);
        }
    }

    mod inversion_ops {
        use super::*;

        mod determinant {
            use super::*;

            #[test]
            fn calculate_determinant_of_2x2_matrix() {
                let a = Matrix::new([
                    [1.0, 5.0],
                    [-3.0, 2.0]
                ]);

                assert_eq!(a.determinant(), 17.0);
            }

            #[test]
            fn calculate_determinant_of_3x3_matrix() {
                let a = Matrix::new([
                    [1.0, 2.0, 6.0],
                    [-5.0, 8.0, -4.0],
                    [2.0, 6.0, 4.0]
                ]);

                assert_eq!(a.cofactor(0, 0), 56.0);
                assert_eq!(a.cofactor(0, 1), 12.0);
                assert_eq!(a.cofactor(0, 2), -46.0);
                assert_eq!(a.determinant(), -196.0);
            }

            #[test]
            fn calculate_determinant_of_4x4_matrix() {
                let a = Matrix::new([
                    [-2.0, -8.0, 3.0, 5.0],
                    [-3.0, 1.0, 7.0, 3.0],
                    [1.0, 2.0, -9.0, 6.0],
                    [-6.0, 7.0, 7.0, -9.0]
                ]);

                assert_eq!(a.cofactor(0, 0), 690.0);
                assert_eq!(a.cofactor(0, 1), 447.0);
                assert_eq!(a.cofactor(0, 2), 210.0);
                assert_eq!(a.cofactor(0, 3), 51.0);
                assert_eq!(a.determinant(), -4071.0);
            }
        }

        mod submatrix {
            use super::*;

            #[test]
            fn submatrix_of_3x3_is_2x2() {
                let a = Matrix::new([
                    [1.0, 5.0, 0.0],
                    [-3.0, 2.0, 7.0],
                    [0.0, 6.0, -3.0]
                ]);

                assert_eq!(a.submatrix(0, 2), Matrix::new([
                    [-3.0, 2.0],
                    [0.0, 6.0]
                ]));
            }

            #[test]
            fn submatrix_of_4x4_is_3x3() {
                let a = Matrix::new([
                    [-6.0, 1.0, 1.0, 6.0],
                    [-8.0, 5.0, 8.0, 6.0],
                    [-1.0, 0.0, 8.0, 2.0],
                    [-7.0, 1.0, -1.0, 1.0]
                ]);

                assert_eq!(a.submatrix(2, 1), Matrix::new([
                    [-6.0, 1.0, 6.0],
                    [-8.0, 8.0, 6.0],
                    [-7.0, -1.0, 1.0]
                ]));
            }
        }

        mod minor {
            use super::*;

            #[test]
            fn calculate_minor_of_3x3_matrix() {
                let a = Matrix::new([
                    [3.0, 5.0, 0.0],
                    [2.0, -1.0, -7.0],
                    [6.0, -1.0, 5.0]
                ]);

                let b = a.submatrix(1, 0);

                assert_eq!(b.determinant(), 25.0);
                assert_eq!(a.minor(1, 0), 25.0);
            }
        }

        mod cofactor {
            use super::*;

            #[test]
            fn calculate_cofactor_of_3x3_matrix() {
                let a = Matrix::new([
                    [3.0, 5.0, 0.0],
                    [2.0, -1.0, -7.0],
                    [6.0, -1.0, 5.0]
                ]);

                assert_eq!(a.minor(0, 0), -12.0);
                assert_eq!(a.cofactor(0, 0), -12.0);
                assert_eq!(a.minor(1, 0), 25.0);
                assert_eq!(a.cofactor(1, 0), -25.0);
            }
        }

        mod invert {
            use super::*;

            #[test]
            fn test_invertible_matrix_for_invertibility() {
                let a = Matrix::new([
                    [6.0, 4.0, 4.0, 4.0],
                    [5.0, 5.0, 7.0, 6.0],
                    [4.0, -9.0, 3.0, -7.0],
                    [9.0, 1.0, 7.0, -6.0]
                ]);

                assert_eq!(a.determinant(), -2120.0);
                assert_eq!(a.is_invertible(), true);
            }

            #[test]
            fn test_noninvertible_matrix_for_invertibility() {
                let a = Matrix::new([
                    [-4.0, 2.0, -2.0, -3.0],
                    [9.0, 6.0, 2.0, 6.0],
                    [0.0, -5.0, 1.0, -5.0],
                    [0.0, 0.0, 0.0, 0.0]
                ]);

                assert_eq!(a.determinant(), 0.0);
                assert_eq!(a.is_invertible(), false);
            }

            #[test]
            fn calculate_inverse_of_a_matrix() {
                let a = Matrix::new([
                    [-5.0, 2.0, 6.0, -8.0],
                    [1.0, -5.0, 1.0, 8.0],
                    [7.0, 7.0, -6.0, -7.0],
                    [1.0, -3.0, 7.0, 4.0]
                ]);

                let b = a.invert().unwrap();

                assert_eq!(a.determinant(), 532.0);
                assert_eq!(a.cofactor(2, 3), -160.0);
                assert_eq!(b.at(3, 2), -160.0 / 532.0);
                assert_eq!(a.cofactor(3, 2), 105.0);
                assert_eq!(b.at(2, 3), 105.0 / 532.0);
                matrix_approx_equals(b, Matrix::new([
                    [0.21805, 0.45113, 0.24060, -0.04511],
                    [-0.80827, -1.45677, -0.44361, 0.52068],
                    [-0.07895, -0.22368, -0.05263, 0.19737],
                    [-0.52256, -0.81391, -0.30075, 0.30639]
                ]));
            }

            #[test]
            fn calculate_inverse_of_another_matrix() {
                let a = Matrix::new([
                    [8.0, -5.0, 9.0, 2.0],
                    [7.0, 5.0, 6.0, 1.0],
                    [-6.0, 0.0, 9.0, 6.0],
                    [-3.0, 0.0, -9.0, -4.0]
                ]);

                matrix_approx_equals(a.invert().unwrap(), Matrix::new([
                    [-0.15385, -0.15385, -0.28205, -0.53846],
                    [-0.07692, 0.12308, 0.02564, 0.03077],
                    [0.35897, 0.35897, 0.43590, 0.92308],
                    [-0.69231, -0.69231, -0.76923, -1.92308]
                ]));
            }

            #[test]
            fn calculate_inverse_of_a_third_matrix() {
                let a = Matrix::new([
                    [9.0, 3.0, 0.0, 9.0],
                    [-5.0, -2.0, -6.0, -3.0],
                    [-4.0, 9.0, 6.0, 4.0],
                    [-7.0, 6.0, 6.0, 2.0]
                ]);

                matrix_approx_equals(a.invert().unwrap(), Matrix::new([
                    [-0.04074, -0.07778, 0.14444, -0.22222],
                    [-0.07778, 0.03333, 0.36667, -0.33333],
                    [-0.02901, -0.14630, -0.10926, 0.12963],
                    [0.17778, 0.06667, -0.26667, 0.33333]
                ]));
            }

            #[test]
            fn multiply_a_product_by_its_inverse() {
                let a = Matrix::new([
                    [3.0, -9.0, 7.0, 3.0],
                    [3.0, -8.0, 2.0, -9.0],
                    [-4.0, 4.0, 4.0, 1.0],
                    [-6.0, 5.0, -1.0, 1.0]
                ]);

                let b = Matrix::new([
                    [8.0, 2.0, 2.0, 2.0],
                    [3.0, -1.0, 7.0, 0.0],
                    [7.0, 0.0, 5.0, 4.0],
                    [6.0, -2.0, 0.0, 5.0]
                ]);

                let c = &a * &b;

                assert_eq!(&c * &b.invert().unwrap(), a);
            }

            fn matrix_approx_equals<const N: usize>(a: Matrix<N>, b: Matrix<N>) {
                for n in 0..N {
                    for m in 0..N {
                        assert!(f64::abs(a.at(n, m) - b.at(n, m)) < 0.00001)
                    }
                }
            }
        }
    }
}
