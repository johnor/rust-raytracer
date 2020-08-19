#[cfg(test)]
use crate::color::Color;
#[cfg(test)]
use crate::matrix::Mat4x4;
#[cfg(test)]
use crate::tuple::Tuple;

#[cfg(test)]
pub fn assert_near(v1: f64, v2: f64) {
    assert!((v1 - v2).abs() < std::f64::EPSILON);
}

#[cfg(test)]
pub fn assert_f64_near(v1: f64, v2: f64, tol: f64) {
    assert!((v1 - v2).abs() < tol);
}

#[cfg(test)]
pub fn assert_tuple_near(t1: Tuple, t2: Tuple, tol: f64) {
    assert!((t1.x - t2.x).abs() < tol);
    assert!((t1.y - t2.y).abs() < tol);
    assert!((t1.z - t2.z).abs() < tol);
    assert!((t1.w - t2.w).abs() < tol);
}

#[cfg(test)]
pub fn assert_mat4x4_near(a: Mat4x4, b: Mat4x4) {
    for r in 0..Mat4x4::order() {
        for c in 0..Mat4x4::order() {
            assert!((b[r][c] - a[r][c]).abs() < 0.00001);
        }
    }
}

#[cfg(test)]
pub fn assert_color_eq(c1: Color, c2: Color) {
    assert_color_near(c1, c2, std::f64::EPSILON);
}

#[cfg(test)]
pub fn assert_color_near(c1: Color, c2: Color, tol: f64) {
    assert!((c1.r - c2.r).abs() < tol);
    assert!((c1.g - c2.g).abs() < tol);
    assert!((c1.b - c2.b).abs() < tol);
}
