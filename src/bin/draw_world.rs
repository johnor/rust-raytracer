extern crate rust_raytracer;
use rust_raytracer::shape::{ShapeType, ShapeObject};
use rust_raytracer::*;

fn main() {
    let mut floor = ShapeObject::new(ShapeType::Sphere);
    floor.transform = transform::scale(10., 0.01, 10.);
    floor.material.color = color::Color::new(1., 0.9, 0.9);
    floor.material.specular = 0.;

    let mut left_wall = ShapeObject::new(ShapeType::Sphere);
    left_wall.transform = transform::translate(0., 0., 5.0)
        * transform::rotate_y(-std::f64::consts::PI / 4.0)
        * transform::rotate_x(std::f64::consts::PI / 2.0)
        * transform::scale(10., 0.01, 10.);
    left_wall.material = floor.material;

    let mut right_wall = ShapeObject::new(ShapeType::Sphere);
    right_wall.transform = transform::translate(0., 0., 5.0)
        * transform::rotate_y(std::f64::consts::PI / 4.0)
        * transform::rotate_x(std::f64::consts::PI / 2.0)
        * transform::scale(10., 0.01, 10.);
    right_wall.material = floor.material;

    let mut middle = ShapeObject::new(ShapeType::Sphere);
    middle.transform = transform::translate(-0.5, 1., 0.5);
    middle.material.color = color::Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = ShapeObject::new(ShapeType::Sphere);
    right.transform = transform::translate(1.5, 0.5, -0.5) * transform::scale(0.5, 0.5, 0.5);
    right.material.color = color::Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = ShapeObject::new(ShapeType::Sphere);
    left.transform = transform::translate(-1.5, 0.33, -0.75) * transform::scale(0.33, 0.33, 0.33);
    left.material.color = color::Color::new(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let light_position = tuple::point(-10., 10., -10.);
    let light_color = color::Color::new(1., 1., 1.);
    let light = lights::PointLight::new(light_color, light_position);

    let world = world::World {
        light,
        shapes: vec![floor, left_wall, right_wall, middle, right, left],
    };

    let mut camera = camera::Camera::new(1000, 500, std::f64::consts::PI / 3.);
    camera.transform = camera::view_transform(
        tuple::point(0., 1.5, -5.),
        tuple::point(0., 1., 0.),
        tuple::vector(0., 1., 0.),
    );

    let canvas = camera.render(world);
    canvas.write_ppm("draw_world.ppm".to_string());
}
