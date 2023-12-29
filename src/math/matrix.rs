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

trait Determinant {
    fn determinant(&self) -> f64;
}
trait Submatrix {
    type Output;

    fn submatrix(&self, n: usize, m: usize) -> Self::Output;
}
trait Cofactor: Submatrix where Self::Output: Determinant {
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

        mod minors {
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

        mod cofactors {
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
    }
}
