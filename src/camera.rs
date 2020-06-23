use crate::matrix::Mat4x4;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub hsize: u32,
    pub vsize: u32,
    pub field_of_view: f64,
    pub transform: Mat4x4,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.).tan();
        let aspect = (hsize as f64) / (vsize as f64);
        let half_width: f64;
        let half_height: f64;

        if aspect >= 1. {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.) / (hsize as f64);

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Mat4x4::identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::matrix::Mat4x4;
    use crate::test_utils::assert_near;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f64::consts::PI / 2.;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(hsize, c.hsize);
        assert_eq!(vsize, c.vsize);
        assert_eq!(field_of_view, c.field_of_view);
        assert_eq!(Mat4x4::identity(), c.transform);
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f64::consts::PI / 2.);
        assert_near(0.01, c.pixel_size);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, std::f64::consts::PI / 2.);
        assert_near(0.01, c.pixel_size);
    }
}
