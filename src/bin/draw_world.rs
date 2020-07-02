extern crate rust_raytracer;
use rust_raytracer::*;

fn main() {
    let mut w = world::World::new();

    let light_position = tuple::point(-10., 10., -10.);
    let light_color = color::Color::new(1., 1., 1.);
    w.light = lights::PointLight::new(light_color, light_position);

    let floor = w.shape_handler.create_sphere();
    w.shape_handler
        .add_transform(floor, transform::scale(10., 0.01, 10.));
    w.shape_handler.material_mut(floor).color = color::Color::new(1., 0.9, 0.9);
    w.shape_handler.material_mut(floor).specular = 0.;

    let left_wall = w.shape_handler.create_sphere();
    let left_wall_trans = transform::translate(0., 0., 5.0)
        * transform::rotate_y(-std::f64::consts::PI / 4.0)
        * transform::rotate_x(std::f64::consts::PI / 2.0)
        * transform::scale(10., 0.01, 10.);
    w.shape_handler.add_transform(left_wall, left_wall_trans);
    w.shape_handler
        .set_material(left_wall, w.shape_handler.material(floor));

    let right_wall = w.shape_handler.create_sphere();
    let right_wall_trans = transform::translate(0., 0., 5.0)
        * transform::rotate_y(std::f64::consts::PI / 4.0)
        * transform::rotate_x(std::f64::consts::PI / 2.0)
        * transform::scale(10., 0.01, 10.);
    w.shape_handler.add_transform(right_wall, right_wall_trans);
    w.shape_handler
        .set_material(right_wall, w.shape_handler.material(floor));

    let middle = w.shape_handler.create_sphere();
    w.shape_handler
        .add_transform(middle, transform::translate(-0.5, 1., 0.5));
    w.shape_handler.material_mut(middle).color = color::Color::new(0.1, 1.0, 0.5);
    w.shape_handler.material_mut(middle).diffuse = 0.7;
    w.shape_handler.material_mut(middle).specular = 0.3;

    let right = w.shape_handler.create_sphere();
    w.shape_handler.add_transform(
        right,
        transform::translate(1.5, 0.5, -0.5) * transform::scale(0.5, 0.5, 0.5),
    );
    w.shape_handler.material_mut(right).color = color::Color::new(0.5, 1.0, 0.1);
    w.shape_handler.material_mut(right).diffuse = 0.7;
    w.shape_handler.material_mut(right).specular = 0.3;

    let left = w.shape_handler.create_sphere();
    w.shape_handler.add_transform(
        left,
        transform::translate(-1.5, 0.33, -0.75) * transform::scale(0.33, 0.33, 0.33),
    );
    w.shape_handler.material_mut(left).color = color::Color::new(1., 0.8, 0.1);
    w.shape_handler.material_mut(left).diffuse = 0.7;
    w.shape_handler.material_mut(left).specular = 0.3;

    let mut camera = camera::Camera::new(1000, 500, std::f64::consts::PI / 3.);
    camera.transform = camera::view_transform(
        tuple::point(0., 1.5, -5.),
        tuple::point(0., 1., 0.),
        tuple::vector(0., 1., 0.),
    );

    let canvas = camera.render(w);
    canvas.write_ppm("draw_world.ppm".to_string());
}
