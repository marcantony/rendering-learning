use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use super::{
    matrix::{Matrix, SquareMatrix},
    vector::Vec3d,
};

#[derive(Debug, Clone)]
pub struct Point3d(Matrix<4, 1>);

impl Point3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3d(Matrix::new([[x], [y], [z], [1.0]]))
    }

    pub fn x(&self) -> f64 {
        self.0.at(0, 0)
    }

    pub fn y(&self) -> f64 {
        self.0.at(1, 0)
    }

    pub fn z(&self) -> f64 {
        self.0.at(2, 0)
    }

    pub fn w(&self) -> f64 {
        self.0.at(3, 0)
    }
}

impl TryFrom<Matrix<4, 1>> for Point3d {
    type Error = String;

    fn try_from(value: Matrix<4, 1>) -> Result<Self, Self::Error> {
        let w = value.at(3, 0);
        if w != 1.0 {
            Err(format!("A point should have w=1.0. Found w={} instead!", w))
        } else {
            Ok(Point3d(value))
        }
    }
}

impl PartialEq for Point3d {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Add<&Vec3d> for &Point3d {
    type Output = Point3d;

    fn add(self, rhs: &Vec3d) -> Self::Output {
        Point3d::new(
            &self.x() + &rhs.x(),
            &self.y() + &rhs.y(),
            &self.z() + &rhs.z(),
        )
    }
}

impl Sub for &Point3d {
    type Output = Vec3d;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d::new(
            &self.x() - &rhs.x(),
            &self.y() - &rhs.y(),
            &self.z() - &rhs.z(),
        )
    }
}

impl Sub<&Vec3d> for &Point3d {
    type Output = Point3d;

    fn sub(self, rhs: &Vec3d) -> Self::Output {
        Point3d::new(
            &self.x() - &rhs.x(),
            &self.y() - &rhs.y(),
            &self.z() - &rhs.z(),
        )
    }
}

impl Mul<&Point3d> for &SquareMatrix<4> {
    type Output = Result<Point3d, String>;

    fn mul(self, rhs: &Point3d) -> Self::Output {
        let output_data = self * &rhs.0;
        Point3d::try_from(output_data)
    }
}

impl Display for Point3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point3d({}, {}, {})", self.x(), self.y(), self.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_tuple_with_w_1() {
        let point = Point3d::new(4.3, -4.2, 3.1);

        assert_eq!(point.x(), 4.3);
        assert_eq!(point.y(), -4.2);
        assert_eq!(point.z(), 3.1);
        assert_eq!(point.w(), 1.0);
    }

    #[test]
    fn points_with_same_vals_are_equal() {
        assert_eq!(Point3d::new(1.0, 2.0, 3.0), Point3d::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn points_with_different_vals_are_not_equal() {
        let p = Point3d::new(1.0, 2.0, 3.0);
        assert_ne!(p, Point3d::new(1.5, 2.0, 3.0));
        assert_ne!(p, Point3d::new(1.0, 2.5, 3.0));
        assert_ne!(p, Point3d::new(1.0, 2.0, 3.5));
    }

    #[test]
    fn points_with_nan_vals_are_not_equal() {
        assert_ne!(
            Point3d::new(f64::NAN, 2.0, 3.0),
            Point3d::new(f64::NAN, 2.0, 3.0)
        );
    }

    mod ops {
        use super::*;

        mod arithmetic {
            use super::*;

            #[test]
            fn add_vector_to_point() {
                let p = Point3d::new(3.0, -2.0, 5.0);
                let v = Vec3d::new(-2.0, 3.0, 1.0);
                assert_eq!(&p + &v, Point3d::new(1.0, 1.0, 6.0));
            }

            #[test]
            fn subtract_two_points() {
                let p1 = Point3d::new(3.0, 2.0, 1.0);
                let p2 = Point3d::new(5.0, 6.0, 7.0);
                assert_eq!(&p1 - &p2, Vec3d::new(-2.0, -4.0, -6.0));
            }

            #[test]
            fn subtract_vector_from_point() {
                let p = Point3d::new(3.0, 2.0, 1.0);
                let v = Vec3d::new(5.0, 6.0, 7.0);
                assert_eq!(&p - &v, Point3d::new(-2.0, -4.0, -6.0));
            }
        }
    }

    mod matrix {
        use super::*;

        #[test]
        fn multiply_matrix_by_point() {
            let a = Matrix::new([
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);

            let b = Point3d::new(1.0, 2.0, 3.0);

            assert_eq!(&a * &b, Ok(Point3d::new(18.0, 24.0, 33.0)));
        }
    }
}
