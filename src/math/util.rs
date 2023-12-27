pub fn are_equal(a: f64, b: f64) -> bool {
    !a.is_nan() && !b.is_nan() && (a - b).abs() < f64::EPSILON
}
