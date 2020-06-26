use crate::matrix::Mat4x4;
use crate::ray::Ray;
use crate::tuple::point;

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
    pub fn ray_for_pixel(&self, px: u32, py: u32) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inv = self
            .transform
            .inverse()
            .expect("Could not get the inverse camera transform");
        let pixel = inv * point(world_x, world_y, -1.);
        let origin = inv * point(0., 0., 0.);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::Camera;
    use crate::matrix::Mat4x4;
    use crate::test_utils::{assert_near, assert_tuple_near};
    use crate::transform::{rotate_y, translate};
    use crate::tuple::{point, vector};

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

    #[test]
    fn constructing_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::PI / 2.);
        let r = c.ray_for_pixel(100, 50);
        assert_tuple_near(point(0., 0., 0.), r.origin, 0.00001);
        assert_tuple_near(vector(0., 0., -1.), r.direction, 0.00001);
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::PI / 2.);
        let r = c.ray_for_pixel(0, 0);
        assert_tuple_near(point(0., 0., 0.), r.origin, 0.00001);
        assert_tuple_near(vector(0.66519, 0.33259, -0.66851), r.direction, 0.00001);
    }

    #[test]
    fn constructing_ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, std::f64::consts::PI / 2.);
        c.transform = rotate_y(std::f64::consts::PI / 4.) * translate(0., -2., 5.);
        let r = c.ray_for_pixel(100, 50);
        assert_tuple_near(point(0., 2., -5.), r.origin, 0.00001);
        assert_tuple_near(
            vector(2.0_f64.sqrt() / 2., 0., -2.0_f64.sqrt() / 2.),
            r.direction,
            0.00001,
        );
    }
}
