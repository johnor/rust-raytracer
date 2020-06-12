use crate::matrix::Mat4x4;

pub fn translation(x: f64, y: f64, z: f64) -> Mat4x4 {
    let mut t = Mat4x4::identity();
    t[0][3] = x;
    t[1][3] = y;
    t[2][3] = z;
    t
}

#[cfg(test)]
mod tests {
    use crate::transform::translation;
    use crate::tuple::{point, vector};

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let t = translation(5., -3., 2.);
        assert_eq!(point(2., 1., 7.), t * point(-3., 4., 5.))
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let t = translation(5., -3., 2.);
        let it = t.inverse().unwrap();
        assert_eq!(point(-8., 7., 3.), it * point(-3., 4., 5.))
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let t = translation(5., -3., 2.);
        let v = vector(-3., 4., 5.);
        assert_eq!(v, t * v)
    }
}
