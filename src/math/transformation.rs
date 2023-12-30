use super::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix<4> {
    Matrix::new([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix<4> {
    Matrix::new([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::tuple::Tuple3;

    mod translation {
        use super::*;

        #[test]
        fn multiplying_by_a_transformation_matrix() {
            let transform = translation(5.0, -3.0, 2.0);
            let p = Tuple3::point(-3.0, 4.0, 5.0);

            assert_eq!(&transform * &p, Tuple3::point(2.0, 1.0, 7.0));
        }

        #[test]
        fn multiplying_by_inverse_of_a_transformation_matrix() {
            let transform = translation(5.0, -3.0, 2.0);
            let inv = transform.invert().unwrap();
            let p = Tuple3::point(-3.0, 4.0, 5.0);

            assert_eq!(&inv * &p, Tuple3::point(-8.0, 7.0, 3.0));
        }

        #[test]
        fn translation_does_not_affect_vectors() {
            let transform = translation(5.0, -3.0, 2.0);
            let v = Tuple3::vec(-3.0, 4.0, 5.0);

            assert_eq!(&transform * &v, v);
        }
    }

    mod scaling {
        use super::*;

        #[test]
        fn scaling_matrix_applied_to_point() {
            let transform = scaling(2.0, 3.0, 4.0);
            let p = Tuple3::point(-4.0, 6.0, 8.0);

            assert_eq!(&transform * &p, Tuple3::point(-8.0, 18.0, 32.0));
        }

        #[test]
        fn scaling_matrix_applied_to_vector() {
            let transform = scaling(2.0, 3.0, 4.0);
            let v = Tuple3::vec(-4.0, 6.0, 8.0);

            assert_eq!(&transform * &v, Tuple3::vec(-8.0, 18.0, 32.0));
        }

        #[test]
        fn multiply_by_inverse_of_scaling_matrix() {
            let transform = scaling(2.0, 3.0, 4.0);
            let inv = transform.invert().unwrap();
            let v = Tuple3::vec(-4.0, 6.0, 8.0);

            assert_eq!(&inv * &v, Tuple3::vec(-2.0, 2.0, 2.0));
        }

        #[test]
        fn reflection_is_scaling_by_a_negative_value() {
            let transform = scaling(-1.0, 1.0, 1.0);
            let p = Tuple3::point(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Tuple3::point(-2.0, 3.0, 4.0));
        }
    }
}
