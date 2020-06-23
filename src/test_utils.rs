#[cfg(test)]
pub fn assert_near(v1: f64, v2: f64) {
    assert!((v1 - v2).abs() < std::f64::EPSILON);
}
