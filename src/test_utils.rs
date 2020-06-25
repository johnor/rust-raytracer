#[cfg(test)]
use crate::tuple::Tuple;

#[cfg(test)]
pub fn assert_near(v1: f64, v2: f64) {
    assert!((v1 - v2).abs() < std::f64::EPSILON);
}

#[cfg(test)]
pub fn assert_tuple_near(t1: Tuple, t2: Tuple, tol: f64) {
    assert!((t1.x - t2.x).abs() < tol);
    assert!((t1.y - t2.y).abs() < tol);
    assert!((t1.z - t2.z).abs() < tol);
    assert!((t1.w - t2.w).abs() < tol);
}
