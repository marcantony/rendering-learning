#[derive(Debug)]
pub struct Tuple3((f64, f64, f64, f64));

impl Tuple3 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple3((x, y, z, w))
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple3((x, y, z, 1.0))
    }

    pub fn vec(x: f64, y: f64, z: f64) -> Self {
        Tuple3((x, y, z, 0.0))
    }

    pub fn x(&self) -> f64 {
        self.0.0
    }

    pub fn y(&self) -> f64 {
        self.0.1
    }

    pub fn z(&self) -> f64 {
        self.0.2
    }

    pub fn w(&self) -> f64 {
        self.0.3
    }

    pub fn is_point(&self) -> bool {
        self.w() == 1.0
    }

    pub fn is_vec(&self) -> bool {
        self.w() == 0.0
    }
}

fn are_equal(a: f64, b: f64) -> bool {
    !a.is_nan() && !b.is_nan() && (a - b).abs() < f64::EPSILON
}

impl PartialEq for Tuple3 {
    fn eq(&self, other: &Self) -> bool {
        are_equal(self.x(), other.x()) &&
        are_equal(self.y(), other.y()) &&
        are_equal(self.z(), other.z()) &&
        are_equal(self.w(), other.w())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_point() {
        let tuple = Tuple3::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(tuple.x(), 4.3);
        assert_eq!(tuple.y(), -4.2);
        assert_eq!(tuple.z(), 3.1);
        assert_eq!(tuple.w(), 1.0);
        assert!(tuple.is_point());
        assert!(!tuple.is_vec())
    }

    #[test]
    fn tuple_with_w_0_is_vec() {
        let tuple = Tuple3::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(tuple.x(), 4.3);
        assert_eq!(tuple.y(), -4.2);
        assert_eq!(tuple.z(), 3.1);
        assert_eq!(tuple.w(), 0.0);
        assert!(!tuple.is_point());
        assert!(tuple.is_vec())
    }

    #[test]
    fn tuples_with_same_vals_are_equal() {
        assert_eq!(Tuple3::new(1.0, 2.0, 3.0, 4.0), Tuple3::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn tuples_with_different_vals_are_not_equal() {
        let t = Tuple3::new(1.0, 2.0, 3.0, 4.0);
        assert_ne!(t, Tuple3::new(1.5, 2.0, 3.0, 4.0));
        assert_ne!(t, Tuple3::new(1.0, 2.5, 3.0, 4.0));
        assert_ne!(t, Tuple3::new(1.0, 2.0, 3.5, 4.0));
        assert_ne!(t, Tuple3::new(1.0, 2.0, 3.0, 4.5));
    }

    #[test]
    fn tuples_with_nan_vals_are_not_equal() {
        assert_ne!(Tuple3::new(f64::NAN, 2.0, 3.0, 4.0), Tuple3::new(f64::NAN, 2.0, 3.0, 4.0));
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = Tuple3::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple3::new(4.0, -4.0, 3.0, 1.0))
    }

    #[test]
    fn vec_creates_tuple_with_w_0() {
        let p = Tuple3::vec(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple3::new(4.0, -4.0, 3.0, 0.0))
    }
}
