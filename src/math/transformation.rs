use core::f64;

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

pub fn rotation_x(radians: f64) -> Matrix<4> {
    let (s, c) = f64::sin_cos(radians);
    Matrix::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, c, -s, 0.0],
        [0.0, s, c, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn rotation_y(radians: f64) -> Matrix<4> {
    let (s, c) = f64::sin_cos(radians);
    Matrix::new([
        [c, 0.0, s, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-s, 0.0, c, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn rotation_z(radians: f64) -> Matrix<4> {
    let (s, c) = f64::sin_cos(radians);
    Matrix::new([
        [c, -s, 0.0, 0.0],
        [s, c, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix<4> {
    Matrix::new([
        [1.0, x_y, x_z, 0.0],
        [y_x, 1.0, y_z, 0.0],
        [z_x, z_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn sequence(transformations: &[Matrix<4>]) -> Matrix<4> {
    transformations.iter().fold(Matrix::<4>::identity(), |acc, t| t * &acc)
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

    mod rotation {
        use super::*;

        use std::f64;

        #[test]
        fn rotating_point_around_x_axis() {
            let p = Tuple3::point(0.0, 1.0, 0.0);
            let half_quarter = rotation_x(f64::consts::FRAC_PI_4);
            let full_quarter = rotation_x(f64::consts::FRAC_PI_2);

            assert_eq!(&half_quarter * &p, Tuple3::point(0.0, f64::consts::SQRT_2 / 2.0, f64::consts::SQRT_2 / 2.0));
            assert_eq!(&full_quarter * &p, Tuple3::point(0.0, 0.0, 1.0));
        }

        #[test]
        fn inverse_of_x_rotation_rotates_in_opposite_direction() {
            let p = Tuple3::point(0.0, 1.0, 0.0);
            let half_quarter = rotation_x(f64::consts::FRAC_PI_4);
            let inv = half_quarter.invert().unwrap();

            assert_eq!(&inv * &p, Tuple3::point(0.0, f64::consts::SQRT_2 / 2.0, -f64::consts::SQRT_2 / 2.0));
        }

        #[test]
        fn rotating_point_around_y_axis() {
            let p = Tuple3::point(0.0, 0.0, 1.0);
            let half_quarter = rotation_y(f64::consts::FRAC_PI_4);
            let full_quarter = rotation_y(f64::consts::FRAC_PI_2);

            assert_eq!(&half_quarter * &p, Tuple3::point(f64::consts::SQRT_2 / 2.0, 0.0, f64::consts::SQRT_2 / 2.0));
            assert_eq!(&full_quarter * &p, Tuple3::point(1.0, 0.0, 0.0));
        }

        #[test]
        fn rotating_point_around_z_axis() {
            let p = Tuple3::point(0.0, 1.0, 0.0);
            let half_quarter = rotation_z(f64::consts::FRAC_PI_4);
            let full_quarter = rotation_z(f64::consts::FRAC_PI_2);

            assert_eq!(&half_quarter * &p, Tuple3::point(-f64::consts::SQRT_2 / 2.0, f64::consts::SQRT_2 / 2.0, 0.0));
            assert_eq!(&full_quarter * &p, Tuple3::point(-1.0, 0.0, 0.0));
        }
    }

    mod shearing {
        use super::*;

        #[test]
        fn shearing_transformation_moves_x_in_proportion_to_y() {
            let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let p = Tuple3::point(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Tuple3::point(5.0, 3.0, 4.0));
        }

        #[test]
        fn shearing_transformation_moves_x_in_proportion_to_z() {
            let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
            let p = Tuple3::point(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Tuple3::point(6.0, 3.0, 4.0));
        }

        #[test]
        fn shearing_transformation_moves_y_in_proportion_to_x() {
            let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
            let p = Tuple3::point(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Tuple3::point(2.0, 5.0, 4.0));
        }

        #[test]
        fn shearing_transformation_moves_y_in_proportion_to_z() {
            let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            let p = Tuple3::point(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Tuple3::point(2.0, 7.0, 4.0));
        }

        #[test]
        fn shearing_transformation_moves_z_in_proportion_to_x() {
            let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            let p = Tuple3::point(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Tuple3::point(2.0, 3.0, 6.0));
        }

        #[test]
        fn shearing_transformation_moves_z_in_proportion_to_y() {
            let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
            let p = Tuple3::point(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Tuple3::point(2.0, 3.0, 7.0));
        }
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple3::point(1.0, 0.0, 1.0);
        let a = rotation_x(f64::consts::FRAC_PI_2);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = &a * &p;
        assert_eq!(p2, Tuple3::point(1.0, -1.0, 0.0));

        let p3 = &b * &p2;
        assert_eq!(p3, Tuple3::point(5.0, -5.0, 0.0));

        let p4 = &c * &p3;
        assert_eq!(p4, Tuple3::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple3::point(1.0, 0.0, 1.0);
        let a = rotation_x(f64::consts::FRAC_PI_2);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = &(&c * &b) * &a;

        assert_eq!(&t * &p, Tuple3::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn sequencing_transformations_applies_in_the_correct_order() {
        let p = Tuple3::point(1.0, 0.0, 1.0);
        let a = rotation_x(f64::consts::FRAC_PI_2);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = sequence(&[a, b, c]);

        assert_eq!(&t * &p, Tuple3::point(15.0, 0.0, 7.0));
    }
}
