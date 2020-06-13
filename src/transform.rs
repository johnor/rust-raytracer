use crate::matrix::Mat4x4;

pub fn translate(x: f64, y: f64, z: f64) -> Mat4x4 {
    let mut m = Mat4x4::identity();
    m[0][3] = x;
    m[1][3] = y;
    m[2][3] = z;
    m
}

pub fn scale(x: f64, y: f64, z: f64) -> Mat4x4 {
    let mut m = Mat4x4::identity();
    m[0][0] = x;
    m[1][1] = y;
    m[2][2] = z;
    m
}

#[cfg(test)]
mod tests {
    use crate::transform::{scale, translate};
    use crate::tuple::{point, vector};

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let m = translate(5., -3., 2.);
        assert_eq!(point(2., 1., 7.), m * point(-3., 4., 5.))
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let m = translate(5., -3., 2.);
        let im = m.inverse().unwrap();
        assert_eq!(point(-8., 7., 3.), im * point(-3., 4., 5.))
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let m = translate(5., -3., 2.);
        let v = vector(-3., 4., 5.);
        assert_eq!(v, m * v)
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let m = scale(2., 3., 4.);
        assert_eq!(point(-8., 18., 32.), m * point(-4., 6., 8.))
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let m = scale(2., 3., 4.);
        assert_eq!(vector(-8., 18., 32.), m * vector(-4., 6., 8.))
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let m = scale(2., 3., 4.);
        let im = m.inverse().unwrap();
        assert_eq!(vector(-2., 2., 2.), im * vector(-4., 6., 8.))
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let m = scale(-1., 1., 1.);
        assert_eq!(point(-2., 3., 4.), m * point(2., 3., 4.))
    }
}
