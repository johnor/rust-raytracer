extern crate rust_raytracer;
use rust_raytracer::*;

fn main() {
    let mut floor = shape::Shape::new(shape::ShapeType::Plane);
    let floor_pattern = patterns::CheckerPattern::new(color::Color::white(), color::Color::black());
    floor.transform = transform::translate(0., 0.0, 0.);
    floor.material.color = color::Color::new(0.2, 0.8, 0.2);
    floor.material.reflective = 0.5;
    floor.material.pattern = Some(patterns::Pattern::Checker(floor_pattern));

    let mut middle = shape::Shape::new(shape::ShapeType::Sphere);
    middle.transform = transform::translate(-0.5, 1., 0.5);
    middle.material.color = color::Color::new(1., 0.0, 0.2);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.reflective = 0.4;

    let mut right = shape::Shape::new(shape::ShapeType::Sphere);
    right.transform = transform::translate(1.5, 0.5, -0.5) * transform::scale(0.5, 0.5, 0.5);
    right.material.color = color::Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = shape::Shape::new(shape::ShapeType::Sphere);
    left.transform = transform::translate(-1.5, 0.33, -0.75) * transform::scale(0.33, 0.33, 0.33);
    left.material.color = color::Color::new(1., 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let light_position = tuple::point(-10., 10., -10.);
    let light_color = color::Color::new(1., 1., 1.);
    let light = lights::PointLight::new(light_color, light_position);

    let world = world::World {
        light,
        shapes: vec![floor, middle, right, left],
    };

    let mut camera = camera::Camera::new(2000, 1000, std::f64::consts::PI / 3.);
    camera.transform = camera::view_transform(
        tuple::point(0., 1.0, -5.),
        tuple::point(0., 1., 0.),
        tuple::vector(0., 1., 0.),
    );

    let canvas = camera.render(world);
    canvas.write_ppm("draw_planes.ppm".to_string());
}
