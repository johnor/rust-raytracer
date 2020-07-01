use crate::canvas::Canvas;
use crate::matrix::Mat4x4;
use crate::ray::Ray;
use crate::transform::translate;
use crate::tuple::{point, Tuple};
use crate::world::World;

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

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);
        for y in 0..image.height() - 1 {
            for x in 0..image.width() - 1 {
                let ray = self.ray_for_pixel(x as u32, y as u32);
                let color = world.color_at(ray);
                image.set_pixel(x as usize, y as usize, color);
            }
        }
        image
    }
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Mat4x4 {
    let forw = (to - from).normalize();
    let left = forw.cross(up.normalize());
    let true_up = left.cross(forw);
    Mat4x4::new([
        [left.x, left.y, left.z, 0.],
        [true_up.x, true_up.y, true_up.z, 0.],
        [-forw.x, -forw.y, -forw.z, 0.],
        [0., 0., 0., 1.],
    ]) * translate(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use crate::camera::{view_transform, Camera};
    use crate::color::Color;
    use crate::matrix::Mat4x4;
    use crate::test_utils::{
        assert_color_near, assert_mat4x4_near, assert_near, assert_tuple_near,
    };
    use crate::transform::{rotate_y, scale, translate};
    use crate::tuple::{point, vector};
    use crate::world::World;

    #[test]
    fn view_transformation_matrix_for_default_orientation() {
        let from = point(0., 0., 0.);
        let to = point(0., 0., -1.);
        let up = vector(0., 1., 0.);
        let t = view_transform(from, to, up);
        assert_mat4x4_near(Mat4x4::identity(), t);
    }

    #[test]
    fn view_transformation_matrix_looking_in_positive_z_direction() {
        let from = point(0., 0., 0.);
        let to = point(0., 0., 1.);
        let up = vector(0., 1., 0.);
        let t = view_transform(from, to, up);
        assert_mat4x4_near(scale(-1., 1., -1.), t);
    }

    #[test]
    fn view_transform_moves_the_world() {
        let from = point(0., 0., 8.);
        let to = point(0., 0., 0.);
        let up = vector(0., 1., 0.);
        let t = view_transform(from, to, up);
        assert_mat4x4_near(translate(0., 0., -8.), t);
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = point(1., 3., 2.);
        let to = point(4., -2., 8.);
        let up = vector(1., 1., 0.);
        let t = view_transform(from, to, up);
        let et = Mat4x4::new([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ]);
        assert_mat4x4_near(et, t);
    }

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

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, std::f64::consts::PI / 2.);
        let from = point(0., 0., -5.0);
        let to = point(0., 0., 0.);
        let up = vector(0., 1., 0.);
        c.transform = view_transform(from, to, up);
        let image = c.render(w);
        assert_color_near(
            Color::new(0.38066, 0.47583, 0.2855),
            image.get_pixel(5, 5),
            0.00001,
        );
    }
}
