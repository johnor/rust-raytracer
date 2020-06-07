mod canvas;
mod color;
mod matrix;
mod tuple;

use tuple::Tuple;

struct Projectile {
    pub pos: Tuple,
    pub vel: Tuple,
}

struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        pos: proj.pos + proj.vel,
        vel: proj.vel + env.gravity + env.wind,
    }
}

fn main() {
    let mut proj = Projectile {
        pos: tuple::point(0.0, 1.0, 0.0),
        vel: tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let env = Environment {
        gravity: tuple::vector(0.0, -0.1, 0.0),
        wind: tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut canvas = canvas::Canvas::new(900, 560);
    let mut ticks = 0;

    let red = color::Color::new(1.0, 0.0, 0.0);

    while proj.pos.y > 0.0 {
        println!("Projectile pos: {:?}", proj.pos);
        canvas.set_pixel(
            proj.pos.x.round() as usize,
            canvas.height() - proj.pos.y.round() as usize,
            red,
        );
        proj = tick(&env, proj);
        ticks += 1;
    }
    println!("Ticks: {}", ticks);
    canvas.write_ppm("projectile.ppm".to_string());
}
