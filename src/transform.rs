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

pub fn rotate_x(r: f64) -> Mat4x4 {
    Mat4x4::new([
        [1., 0., 0., 0.],
        [0., f64::cos(r), -f64::sin(r), 0.],
        [0., f64::sin(r), f64::cos(r), 0.],
        [0., 0., 0., 1.],
    ])
}

pub fn rotate_y(r: f64) -> Mat4x4 {
    Mat4x4::new([
        [f64::cos(r), 0., f64::sin(r), 0.],
        [0., 1.0, 0., 0.],
        [-f64::sin(r), 0., f64::cos(r), 0.],
        [0., 0., 0., 1.],
    ])
}
pub fn rotate_z(r: f64) -> Mat4x4 {
    Mat4x4::new([
        [f64::cos(r), -f64::sin(r), 0., 0.],
        [f64::sin(r), f64::cos(r), 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ])
}

#[cfg(test)]
mod tests {
    use crate::transform::{rotate_x, rotate_y, rotate_z, scale, translate};
    use crate::tuple::test_utils::assert_tuple_eq;
    use crate::tuple::{point, vector};

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let m = translate(5., -3., 2.);
        assert_tuple_eq(point(2., 1., 7.), m * point(-3., 4., 5.))
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let m = translate(5., -3., 2.);
        let im = m.inverse().unwrap();
        assert_tuple_eq(point(-8., 7., 3.), im * point(-3., 4., 5.))
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let m = translate(5., -3., 2.);
        let v = vector(-3., 4., 5.);
        assert_tuple_eq(v, m * v)
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let m = scale(2., 3., 4.);
        assert_tuple_eq(point(-8., 18., 32.), m * point(-4., 6., 8.))
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let m = scale(2., 3., 4.);
        assert_tuple_eq(vector(-8., 18., 32.), m * vector(-4., 6., 8.))
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let m = scale(2., 3., 4.);
        let im = m.inverse().unwrap();
        assert_tuple_eq(vector(-2., 2., 2.), im * vector(-4., 6., 8.))
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let m = scale(-1., 1., 1.);
        assert_tuple_eq(point(-2., 3., 4.), m * point(2., 3., 4.))
    }

    #[test]
    fn rotate_around_x_axis() {
        let p = point(0., 1., 0.);
        let mhq = rotate_x(std::f64::consts::PI / 4.);
        let mfq = rotate_x(std::f64::consts::PI / 2.);
        assert_tuple_eq(point(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.), mhq * p);
        assert_tuple_eq(point(0., 0., 1.), mfq * p);
    }

    #[test]
    fn inverse_rotate_around_x() {
        let p = point(0., 1., 0.);
        let mhq = rotate_x(std::f64::consts::PI / 4.);
        assert_tuple_eq(
            point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.),
            mhq.inverse().unwrap() * p,
        );
    }

    #[test]
    fn rotate_around_y_axis() {
        let p = point(0., 0., 1.);
        let mhq = rotate_y(std::f64::consts::PI / 4.);
        let mfq = rotate_y(std::f64::consts::PI / 2.);
        assert_tuple_eq(point(2_f64.sqrt() / 2., 0., 2_f64.sqrt() / 2.), mhq * p);
        assert_tuple_eq(point(1., 0., 0.), mfq * p);
    }

    #[test]
    fn rotate_around_z_axis() {
        let p = point(0., 1., 0.);
        let mhq = rotate_z(std::f64::consts::PI / 4.);
        let mfq = rotate_z(std::f64::consts::PI / 2.);
        assert_tuple_eq(point(-2_f64.sqrt() / 2., 2_f64.sqrt() / 2., 0.), mhq * p);
        assert_tuple_eq(point(-1., 0., 0.), mfq * p);
    }
}
