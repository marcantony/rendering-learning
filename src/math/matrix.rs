use std::ops::{Add, Deref, Index, IndexMut, Mul, Sub};

use super::util;

#[derive(Debug, Clone)]
pub struct Matrix<const N: usize, const M: usize>([[f64; M]; N]);
pub type SquareMatrix<const N: usize> = Matrix<N, N>;

impl<const N: usize, const M: usize> Matrix<N, M> {
    pub fn new(data: [[f64; M]; N]) -> Self {
        Matrix(data)
    }

    pub fn at(&self, n: usize, m: usize) -> f64 {
        self.0[n][m]
    }

    pub fn put(&mut self, n: usize, m: usize, val: f64) {
        self.0[n][m] = val;
    }

    pub fn transpose(&self) -> Matrix<M, N> {
        let mut data = [[0.0; N]; M];

        for n in 0..N {
            for m in 0..M {
                data[m][n] = self.0[n][m];
            }
        }

        Matrix::new(data)
    }

    fn get_slices(&self) -> Vec<&[f64]> {
        self.0.iter().map(|col| col.as_slice()).collect::<Vec<_>>()
    }
}

impl<const N: usize> SquareMatrix<N> {
    pub fn identity() -> Self {
        let mut data = [[0.0; N]; N];

        for i in 0..N {
            data[i][i] = 1.0;
        }

        Matrix::new(data)
    }

    pub fn determinant(&self) -> f64 {
        determinant(&self.get_slices())
    }

    pub fn minor(&self, n: usize, m: usize) -> f64 {
        minor(&self.get_slices(), n, m)
    }

    pub fn cofactor(&self, n: usize, m: usize) -> f64 {
        cofactor(&self.get_slices(), n, m)
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn invert(&self) -> Option<Self> {
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

fn determinant(data: &[&[f64]]) -> f64 {
    if data.len() == 2 {
        data[0][0] * data[1][1] - data[0][1] * data[1][0]
    } else {
        let mut sum = 0.0;

        for i in 0..data.len() {
            sum += data[0][i] * cofactor(data, 0, i);
        }

        sum
    }
}

fn cofactor(data: &[&[f64]], n: usize, m: usize) -> f64 {
    let minor = minor(data, n, m);
    if (n + m) % 2 == 0 {
        minor
    } else {
        -minor
    }
}

fn minor(data: &[&[f64]], n: usize, m: usize) -> f64 {
    let raw_submatrix_data = submatrix(data, n, m);
    let submatrix_data = raw_submatrix_data
        .iter()
        .map(|row| row.as_slice())
        .collect::<Vec<_>>();
    determinant(submatrix_data.as_slice())
}

fn submatrix(data: &[&[f64]], n: usize, m: usize) -> Vec<Vec<f64>> {
    data.iter()
        .enumerate()
        .filter(|&(i, _)| i != n)
        .map(|(_, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, _)| j != m)
                .map(|(_, &col)| col)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

impl<const N: usize, const M: usize> PartialEq for Matrix<N, M> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .flat_map(|(rhs_rows, lhs_rows)| lhs_rows.iter().zip(rhs_rows))
            .all(|(&lhs, &rhs)| util::are_equal(lhs, rhs))
    }
}

impl<const N: usize, const M: usize> Add for &Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = [[0.0; M]; N];

        for n in 0..N {
            for m in 0..M {
                data[n][m] = self.0[n][m] + rhs.0[n][m];
            }
        }

        Matrix::new(data)
    }
}

impl<const N: usize, const M: usize> Sub for &Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = [[0.0; M]; N];

        for n in 0..N {
            for m in 0..M {
                data[n][m] = self.0[n][m] - rhs.0[n][m];
            }
        }

        Matrix::new(data)
    }
}

impl<const N: usize, const M: usize> Mul<f64> for &Matrix<N, M> {
    type Output = Matrix<N, M>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut data = [[0.0; M]; N];

        for n in 0..N {
            for m in 0..M {
                data[n][m] = self.0[n][m] * rhs;
            }
        }

        Matrix::new(data)
    }
}

impl<const N1: usize, const N2: usize, const M: usize> Mul<&Matrix<M, N2>> for &Matrix<N1, M> {
    type Output = Matrix<N1, N2>;

    fn mul(self, rhs: &Matrix<M, N2>) -> Self::Output {
        let mut output = [[0.0; N2]; N1];

        for n in 0..N1 {
            for m in 0..N2 {
                let mut sum = 0.0;
                for i in 0..M {
                    sum += self.0[n][i] * rhs.0[i][m];
                }
                output[n][m] = sum;
            }
        }

        Matrix::new(output)
    }
}

impl<const N: usize, const M: usize> Index<(usize, usize)> for Matrix<N, M> {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<const N: usize, const M: usize> IndexMut<(usize, usize)> for Matrix<N, M> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InvertibleMatrix<const N: usize> {
    matrix: SquareMatrix<N>,
    inverse: SquareMatrix<N>,
}

impl<const N: usize> InvertibleMatrix<N> {
    pub fn identity() -> Self {
        InvertibleMatrix {
            matrix: SquareMatrix::identity(),
            inverse: SquareMatrix::identity(),
        }
    }

    pub fn inverse(&self) -> &SquareMatrix<N> {
        &self.inverse
    }
}

impl<const N: usize> TryFrom<SquareMatrix<N>> for InvertibleMatrix<N> {
    type Error = String;

    fn try_from(value: SquareMatrix<N>) -> Result<Self, Self::Error> {
        match value.invert() {
            Some(inverse) => Ok(InvertibleMatrix {
                matrix: value,
                inverse,
            }),
            None => Err("Matrix is not invertible.".to_string()),
        }
    }
}

impl<const N: usize> Deref for InvertibleMatrix<N> {
    type Target = SquareMatrix<N>;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl<const N: usize> Default for InvertibleMatrix<N> {
    fn default() -> Self {
        InvertibleMatrix::identity()
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    pub fn assert_matrix_approx_equals<const N: usize>(a: &SquareMatrix<N>, b: &SquareMatrix<N>) {
        for n in 0..N {
            for m in 0..N {
                assert!(util::test_utils::are_within_tolerance(
                    a.at(n, m),
                    b.at(n, m),
                    1e-5
                ));
            }
        }
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
            [13.5, 14.5, 15.5, 16.5],
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
        let m = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(0, 1), 5.0);
        assert_eq!(m.at(1, 0), 1.0);
        assert_eq!(m.at(1, 1), -2.0);
    }

    #[test]
    fn a_3x3_matrix_is_representable() {
        let m = Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(m.at(0, 0), -3.0);
        assert_eq!(m.at(1, 1), -2.0);
        assert_eq!(m.at(2, 2), 1.0);
    }

    #[test]
    fn a_non_square_matrix_is_representable() {
        let m = Matrix::new([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);

        assert_eq!(m.at(0, 0), 1.0);
        assert_eq!(m.at(1, 1), 4.0);
        assert_eq!(m.at(2, 1), 6.0);
    }

    mod equality {
        use super::*;

        #[test]
        fn equality_with_identical_matrices() {
            let a = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ]);

            let b = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ]);

            assert_eq!(a, b);
        }

        #[test]
        fn equality_with_different_matrices() {
            let a = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ]);

            let b = Matrix::new([
                [2.0, 3.0, 4.0, 5.0],
                [6.0, 7.0, 8.0, 9.0],
                [8.0, 7.0, 6.0, 5.0],
                [4.0, 3.0, 2.0, 1.0],
            ]);

            assert_ne!(a, b);
        }

        #[test]
        fn equality_of_identical_non_square_matrices() {
            let a = Matrix::new([[1.0, 2.0]]);

            let b = Matrix::new([[1.0, 2.0]]);

            assert_eq!(a, b);
        }
    }

    mod arithmetic {
        use super::*;

        #[test]
        fn add_matrix_to_another() {
            let a = Matrix::new([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);

            let b = a.clone();

            assert_eq!(&a + &b, Matrix::new([[2.0, 4.0], [6.0, 8.0], [10.0, 12.0]]))
        }

        #[test]
        fn subtract_matrix_from_another() {
            let a = Matrix::new([[3.0, 3.0, 3.0], [3.0, 3.0, 3.0]]);

            let b = Matrix::new([[2.0, 2.0, 2.0], [2.0, 2.0, 2.0]]);

            assert_eq!(&a - &b, Matrix::new([[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]]));
        }

        #[test]
        fn multiply_matrix_by_scalar() {
            let a = Matrix::new([[1.0, 2.0, 3.0]]);

            assert_eq!(&a * 2.0, Matrix::new([[2.0, 4.0, 6.0]]));
        }
    }

    mod multiply {
        use super::*;

        #[test]
        fn multiply_two_matrices() {
            let a = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ]);

            let b = Matrix::new([
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0],
            ]);

            assert_eq!(
                &a * &b,
                Matrix::new([
                    [20.0, 22.0, 50.0, 48.0],
                    [44.0, 54.0, 114.0, 108.0],
                    [40.0, 58.0, 110.0, 102.0],
                    [16.0, 26.0, 46.0, 42.0]
                ])
            );
        }

        #[test]
        fn multiply_matrix_by_identity() {
            let a = Matrix::new([
                [0.0, 1.0, 2.0, 4.0],
                [1.0, 2.0, 4.0, 8.0],
                [2.0, 4.0, 8.0, 16.0],
                [4.0, 8.0, 16.0, 32.0],
            ]);

            let b = Matrix::identity();

            assert_eq!(&a * &b, a);
        }

        #[test]
        fn multiply_non_square_matrices() {
            let a = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);

            let b = Matrix::new([[1.0], [2.0], [3.0]]);

            assert_eq!(&a * &b, Matrix::new([[14.0], [32.0]]));
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
                [0.0, 0.0, 5.0, 8.0],
            ]);

            assert_eq!(
                a.transpose(),
                Matrix::new([
                    [0.0, 9.0, 1.0, 0.0],
                    [9.0, 8.0, 8.0, 0.0],
                    [3.0, 0.0, 5.0, 5.0],
                    [0.0, 8.0, 3.0, 8.0]
                ])
            )
        }

        #[test]
        fn transpose_identity_matrix() {
            let a = SquareMatrix::<4>::identity();

            assert_eq!(a.transpose(), a);
        }

        #[test]
        fn transpose_non_square_matrix() {
            let a = Matrix::new([[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]]);

            assert_eq!(
                a.transpose(),
                Matrix::new([[1.0, 3.0, 5.0], [2.0, 4.0, 6.0]])
            );
        }
    }

    mod inversion_ops {
        use super::*;

        mod determinant {
            use super::*;

            #[test]
            fn calculate_determinant_of_2x2_matrix() {
                let a = Matrix::new([[1.0, 5.0], [-3.0, 2.0]]);

                assert_eq!(a.determinant(), 17.0);
            }

            #[test]
            fn calculate_determinant_of_3x3_matrix() {
                let a = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

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
                    [-6.0, 7.0, 7.0, -9.0],
                ]);

                assert_eq!(a.cofactor(0, 0), 690.0);
                assert_eq!(a.cofactor(0, 1), 447.0);
                assert_eq!(a.cofactor(0, 2), 210.0);
                assert_eq!(a.cofactor(0, 3), 51.0);
                assert_eq!(a.determinant(), -4071.0);
            }
        }

        mod minor {
            use super::*;

            #[test]
            fn calculate_minor_of_3x3_matrix() {
                let a = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

                assert_eq!(a.minor(1, 0), 25.0);
            }
        }

        mod cofactor {
            use super::*;

            #[test]
            fn calculate_cofactor_of_3x3_matrix() {
                let a = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

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
                    [9.0, 1.0, 7.0, -6.0],
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
                    [0.0, 0.0, 0.0, 0.0],
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
                    [1.0, -3.0, 7.0, 4.0],
                ]);

                let b = a.invert().unwrap();

                assert_eq!(a.determinant(), 532.0);
                assert_eq!(a.cofactor(2, 3), -160.0);
                assert_eq!(b.at(3, 2), -160.0 / 532.0);
                assert_eq!(a.cofactor(3, 2), 105.0);
                assert_eq!(b.at(2, 3), 105.0 / 532.0);
                test_utils::assert_matrix_approx_equals(
                    &b,
                    &Matrix::new([
                        [0.21805, 0.45113, 0.24060, -0.04511],
                        [-0.80827, -1.45677, -0.44361, 0.52068],
                        [-0.07895, -0.22368, -0.05263, 0.19737],
                        [-0.52256, -0.81391, -0.30075, 0.30639],
                    ]),
                );
            }

            #[test]
            fn calculate_inverse_of_another_matrix() {
                let a = Matrix::new([
                    [8.0, -5.0, 9.0, 2.0],
                    [7.0, 5.0, 6.0, 1.0],
                    [-6.0, 0.0, 9.0, 6.0],
                    [-3.0, 0.0, -9.0, -4.0],
                ]);

                test_utils::assert_matrix_approx_equals(
                    a.invert().as_ref().unwrap(),
                    &Matrix::new([
                        [-0.15385, -0.15385, -0.28205, -0.53846],
                        [-0.07692, 0.12308, 0.02564, 0.03077],
                        [0.35897, 0.35897, 0.43590, 0.92308],
                        [-0.69231, -0.69231, -0.76923, -1.92308],
                    ]),
                );
            }

            #[test]
            fn calculate_inverse_of_a_third_matrix() {
                let a = Matrix::new([
                    [9.0, 3.0, 0.0, 9.0],
                    [-5.0, -2.0, -6.0, -3.0],
                    [-4.0, 9.0, 6.0, 4.0],
                    [-7.0, 6.0, 6.0, 2.0],
                ]);

                test_utils::assert_matrix_approx_equals(
                    a.invert().as_ref().unwrap(),
                    &Matrix::new([
                        [-0.04074, -0.07778, 0.14444, -0.22222],
                        [-0.07778, 0.03333, 0.36667, -0.33333],
                        [-0.02901, -0.14630, -0.10926, 0.12963],
                        [0.17778, 0.06667, -0.26667, 0.33333],
                    ]),
                );
            }

            #[test]
            fn multiply_a_product_by_its_inverse() {
                let a = Matrix::new([
                    [3.0, -9.0, 7.0, 3.0],
                    [3.0, -8.0, 2.0, -9.0],
                    [-4.0, 4.0, 4.0, 1.0],
                    [-6.0, 5.0, -1.0, 1.0],
                ]);

                let b = Matrix::new([
                    [8.0, 2.0, 2.0, 2.0],
                    [3.0, -1.0, 7.0, 0.0],
                    [7.0, 0.0, 5.0, 4.0],
                    [6.0, -2.0, 0.0, 5.0],
                ]);

                let c = &a * &b;

                assert_eq!(&c * &b.invert().unwrap(), a);
            }
        }
    }

    mod invertible_matrix {
        use super::*;

        #[test]
        fn construct_invertible_matrix() {
            let a = InvertibleMatrix::try_from(Matrix::new([
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ]))
            .unwrap();

            let b = a.inverse();

            assert_eq!(a.determinant(), 532.0);
            assert_eq!(a.cofactor(2, 3), -160.0);
            assert_eq!(b.at(3, 2), -160.0 / 532.0);
            assert_eq!(a.cofactor(3, 2), 105.0);
            assert_eq!(b.at(2, 3), 105.0 / 532.0);
            test_utils::assert_matrix_approx_equals(
                b,
                &Matrix::new([
                    [0.21805, 0.45113, 0.24060, -0.04511],
                    [-0.80827, -1.45677, -0.44361, 0.52068],
                    [-0.07895, -0.22368, -0.05263, 0.19737],
                    [-0.52256, -0.81391, -0.30075, 0.30639],
                ]),
            );
        }

        #[test]
        fn try_constructing_invertible_matrix_from_uninvertible() {
            let a = Matrix::new([
                [-4.0, 2.0, -2.0, -3.0],
                [9.0, 6.0, 2.0, 6.0],
                [0.0, -5.0, 1.0, -5.0],
                [0.0, 0.0, 0.0, 0.0],
            ]);

            let b = InvertibleMatrix::try_from(a);

            assert!(b.is_err());
        }

        #[test]
        fn construct_invertible_matrix_from_identity() {
            let m = InvertibleMatrix::<4>::identity();
            let id = SquareMatrix::<4>::identity();

            assert_eq!(*m, id);
            assert_eq!(m.inverse(), &id);
        }
    }
}
