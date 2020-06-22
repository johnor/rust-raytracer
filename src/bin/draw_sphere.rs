extern crate rust_raytracer;
use crate::rust_raytracer::shape::Shape;
use rust_raytracer::*;

fn main() {
    let canvas_pixels = 300;
    let mut canvas = canvas::Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = sphere::Sphere::new();
    shape.transform =
        transform::rotate_z(std::f64::consts::PI / 4.0) * transform::scale(1.0, 0.5, 1.0);
    shape.material.color = color::Color::new(1., 0.2, 1.);

    let light_position = tuple::point(-10., 10., -10.);
    let light_color = color::Color::new(1., 1., 1.);
    let light = lights::PointLight::new(light_color, light_position);

    let ray_origin = tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let half_wall = wall_size / 2.0;

    for canvas_row in 0..canvas_pixels - 1 {
        let world_y = half_wall - pixel_size * (canvas_row as f64);
        for canvas_col in 0..canvas_pixels - 1 {
            let world_x = -half_wall + pixel_size * (canvas_col as f64);

            let pos_at_wall = tuple::point(world_x, world_y, wall_z);
            let ray = ray::Ray::new(ray_origin, (pos_at_wall - ray_origin).normalize());
            let xs = shape.intersect(ray);
            let hit = intersections::hit(xs);
            if hit.is_some() {
                let point = ray.position(hit.unwrap().t);
                let normal = hit.unwrap().object.normal(point);
                let eye = -ray.direction;
                let color = hit
                    .unwrap()
                    .object
                    .material
                    .lighting(light, point, eye, normal);
                canvas.set_pixel(canvas_col, canvas_row, color);
            }
        }
    }

    canvas.write_ppm("draw_sphere.ppm".to_string());
}
