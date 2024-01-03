use core::f64;

use crate::math::{
    matrix::{Matrix, SquareMatrix},
    point::Point3d,
    vector::Vec3d,
};

pub fn translation(x: f64, y: f64, z: f64) -> SquareMatrix<4> {
    Matrix::new([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scaling(x: f64, y: f64, z: f64) -> SquareMatrix<4> {
    Matrix::new([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_x(radians: f64) -> SquareMatrix<4> {
    let (s, c) = f64::sin_cos(radians);
    Matrix::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, c, -s, 0.0],
        [0.0, s, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_y(radians: f64) -> SquareMatrix<4> {
    let (s, c) = f64::sin_cos(radians);
    Matrix::new([
        [c, 0.0, s, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-s, 0.0, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_z(radians: f64) -> SquareMatrix<4> {
    let (s, c) = f64::sin_cos(radians);
    Matrix::new([
        [c, -s, 0.0, 0.0],
        [s, c, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> SquareMatrix<4> {
    Matrix::new([
        [1.0, x_y, x_z, 0.0],
        [y_x, 1.0, y_z, 0.0],
        [z_x, z_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn sequence(transformations: &[SquareMatrix<4>]) -> SquareMatrix<4> {
    transformations
        .iter()
        .fold(SquareMatrix::<4>::identity(), |acc, t| t * &acc)
}

pub fn view_transform(from: &Point3d, to: &Point3d, up: &Vec3d) -> SquareMatrix<4> {
    let forward = (to - from).norm().unwrap();
    let upn = up.norm().unwrap();
    let left = forward.cross(&upn);
    let true_up = left.cross(&forward);

    let orientation = Matrix::new([
        [left.x(), left.y(), left.z(), 0.0],
        [true_up.x(), true_up.y(), true_up.z(), 0.0],
        [-forward.x(), -forward.y(), -forward.z(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    &orientation * &translation(-from.x(), -from.y(), -from.z())
}

#[cfg(test)]
mod tests {
    use crate::math::{point::Point3d, vector::Vec3d};

    use super::*;

    mod translation {
        use super::*;

        #[test]
        fn multiplying_by_a_transformation_matrix() {
            let transform = translation(5.0, -3.0, 2.0);
            let p = Point3d::new(-3.0, 4.0, 5.0);

            assert_eq!(&transform * &p, Point3d::new(2.0, 1.0, 7.0));
        }

        #[test]
        fn multiplying_by_inverse_of_a_transformation_matrix() {
            let transform = translation(5.0, -3.0, 2.0);
            let inv = transform.invert().unwrap();
            let p = Point3d::new(-3.0, 4.0, 5.0);

            assert_eq!(&inv * &p, Point3d::new(-8.0, 7.0, 3.0));
        }

        #[test]
        fn translation_does_not_affect_vectors() {
            let transform = translation(5.0, -3.0, 2.0);
            let v = Vec3d::new(-3.0, 4.0, 5.0);

            assert_eq!(&transform * &v, v);
        }
    }

    mod scaling {
        use super::*;

        #[test]
        fn scaling_matrix_applied_to_point() {
            let transform = scaling(2.0, 3.0, 4.0);
            let p = Point3d::new(-4.0, 6.0, 8.0);

            assert_eq!(&transform * &p, Point3d::new(-8.0, 18.0, 32.0));
        }

        #[test]
        fn scaling_matrix_applied_to_vector() {
            let transform = scaling(2.0, 3.0, 4.0);
            let v = Vec3d::new(-4.0, 6.0, 8.0);

            assert_eq!(&transform * &v, Vec3d::new(-8.0, 18.0, 32.0));
        }

        #[test]
        fn multiply_by_inverse_of_scaling_matrix() {
            let transform = scaling(2.0, 3.0, 4.0);
            let inv = transform.invert().unwrap();
            let v = Vec3d::new(-4.0, 6.0, 8.0);

            assert_eq!(&inv * &v, Vec3d::new(-2.0, 2.0, 2.0));
        }

        #[test]
        fn reflection_is_scaling_by_a_negative_value() {
            let transform = scaling(-1.0, 1.0, 1.0);
            let p = Point3d::new(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Point3d::new(-2.0, 3.0, 4.0));
        }
    }

    mod rotation {
        use super::*;

        use std::f64;

        #[test]
        fn rotating_point_around_x_axis() {
            let p = Point3d::new(0.0, 1.0, 0.0);
            let half_quarter = rotation_x(f64::consts::FRAC_PI_4);
            let full_quarter = rotation_x(f64::consts::FRAC_PI_2);

            assert_eq!(
                &half_quarter * &p,
                Point3d::new(0.0, f64::consts::SQRT_2 / 2.0, f64::consts::SQRT_2 / 2.0)
            );
            assert_eq!(&full_quarter * &p, Point3d::new(0.0, 0.0, 1.0));
        }

        #[test]
        fn inverse_of_x_rotation_rotates_in_opposite_direction() {
            let p = Point3d::new(0.0, 1.0, 0.0);
            let half_quarter = rotation_x(f64::consts::FRAC_PI_4);
            let inv = half_quarter.invert().unwrap();

            assert_eq!(
                &inv * &p,
                Point3d::new(0.0, f64::consts::SQRT_2 / 2.0, -f64::consts::SQRT_2 / 2.0)
            );
        }

        #[test]
        fn rotating_point_around_y_axis() {
            let p = Point3d::new(0.0, 0.0, 1.0);
            let half_quarter = rotation_y(f64::consts::FRAC_PI_4);
            let full_quarter = rotation_y(f64::consts::FRAC_PI_2);

            assert_eq!(
                &half_quarter * &p,
                Point3d::new(f64::consts::SQRT_2 / 2.0, 0.0, f64::consts::SQRT_2 / 2.0)
            );
            assert_eq!(&full_quarter * &p, Point3d::new(1.0, 0.0, 0.0));
        }

        #[test]
        fn rotating_point_around_z_axis() {
            let p = Point3d::new(0.0, 1.0, 0.0);
            let half_quarter = rotation_z(f64::consts::FRAC_PI_4);
            let full_quarter = rotation_z(f64::consts::FRAC_PI_2);

            assert_eq!(
                &half_quarter * &p,
                Point3d::new(-f64::consts::SQRT_2 / 2.0, f64::consts::SQRT_2 / 2.0, 0.0)
            );
            assert_eq!(&full_quarter * &p, Point3d::new(-1.0, 0.0, 0.0));
        }
    }

    mod shearing {
        use super::*;

        #[test]
        fn shearing_transformation_moves_x_in_proportion_to_y() {
            let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let p = Point3d::new(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Point3d::new(5.0, 3.0, 4.0));
        }

        #[test]
        fn shearing_transformation_moves_x_in_proportion_to_z() {
            let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
            let p = Point3d::new(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Point3d::new(6.0, 3.0, 4.0));
        }

        #[test]
        fn shearing_transformation_moves_y_in_proportion_to_x() {
            let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
            let p = Point3d::new(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Point3d::new(2.0, 5.0, 4.0));
        }

        #[test]
        fn shearing_transformation_moves_y_in_proportion_to_z() {
            let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
            let p = Point3d::new(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Point3d::new(2.0, 7.0, 4.0));
        }

        #[test]
        fn shearing_transformation_moves_z_in_proportion_to_x() {
            let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
            let p = Point3d::new(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Point3d::new(2.0, 3.0, 6.0));
        }

        #[test]
        fn shearing_transformation_moves_z_in_proportion_to_y() {
            let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
            let p = Point3d::new(2.0, 3.0, 4.0);

            assert_eq!(&transform * &p, Point3d::new(2.0, 3.0, 7.0));
        }
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Point3d::new(1.0, 0.0, 1.0);
        let a = rotation_x(f64::consts::FRAC_PI_2);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = &a * &p;
        assert_eq!(p2, Point3d::new(1.0, -1.0, 0.0));

        let p3 = &b * &p2;
        assert_eq!(p3, Point3d::new(5.0, -5.0, 0.0));

        let p4 = &c * &p3;
        assert_eq!(p4, Point3d::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Point3d::new(1.0, 0.0, 1.0);
        let a = rotation_x(f64::consts::FRAC_PI_2);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = &(&c * &b) * &a;

        assert_eq!(&t * &p, Point3d::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn sequencing_transformations_applies_in_the_correct_order() {
        let p = Point3d::new(1.0, 0.0, 1.0);
        let a = rotation_x(f64::consts::FRAC_PI_2);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let t = sequence(&[a, b, c]);

        assert_eq!(&t * &p, Point3d::new(15.0, 0.0, 7.0));
    }

    mod view_transform {
        use super::*;

        #[test]
        fn view_transformation_matrix_for_default_orientation() {
            let from = Point3d::new(0.0, 0.0, 0.0);
            let to = Point3d::new(0.0, 0.0, -1.0);
            let up = Vec3d::new(0.0, 1.0, 0.0);

            let t = view_transform(&from, &to, &up);

            assert_eq!(t, Matrix::identity());
        }

        #[test]
        fn view_transformation_matrix_looking_in_positive_z_direction() {
            let from = Point3d::new(0.0, 0.0, 0.0);
            let to = Point3d::new(0.0, 0.0, 1.0);
            let up = Vec3d::new(0.0, 1.0, 0.0);

            let t = view_transform(&from, &to, &up);

            assert_eq!(t, scaling(-1.0, 1.0, -1.0));
        }

        #[test]
        fn view_transformation_moves_the_world() {
            let from = Point3d::new(0.0, 0.0, 8.0);
            let to = Point3d::new(0.0, 0.0, 0.0);
            let up = Vec3d::new(0.0, 1.0, 0.0);

            let t = view_transform(&from, &to, &up);

            assert_eq!(t, translation(0.0, 0.0, -8.0));
        }

        #[test]
        fn arbitrary_view_transformation() {
            let from = Point3d::new(1.0, 3.0, 2.0);
            let to = Point3d::new(4.0, -2.0, 8.0);
            let up = Vec3d::new(1.0, 1.0, 0.0);

            let t = view_transform(&from, &to, &up);

            let expected = Matrix::new([
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000],
            ]);

            matrix_approx_equals(&t, &expected);
        }

        fn matrix_approx_equals<const N: usize>(a: &SquareMatrix<N>, b: &SquareMatrix<N>) {
            for n in 0..N {
                for m in 0..N {
                    let lhs = a.at(n, m);
                    let rhs = b.at(n, m);
                    assert!(
                        f64::abs(lhs - rhs) < 1e-5,
                        "a[{}][{}]={}, b[{}][{}]={}",
                        n,
                        m,
                        lhs,
                        n,
                        m,
                        rhs
                    );
                }
            }
        }
    }
}
