use super::util;

#[derive(Debug)]
pub struct Matrix<const N: usize>([[f64; N]; N]);

impl<const N: usize> Matrix<N> {
    pub fn new(data: [[f64; N]; N]) -> Self {
        Matrix(data)
    }

    pub fn at(&self, x: usize, y: usize) -> f64 {
        self.0[x][y]
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
}
