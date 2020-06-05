mod canvas;
mod color;
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
    println!("Hello, world!");
    let t = Tuple {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };
    println!("A tuple: {:?}", t);
    println!("Is point: {}, is vector: {}", t.is_point(), t.is_vector());

    let t2 = Tuple {
        x: 2.0,
        w: 2.0,
        ..t
    };
    println!("A tuple: {:?}", t2);
    println!("Is point: {}, is vector: {}", t2.is_point(), t2.is_vector());

    let mut proj = Projectile {
        pos: tuple::point(0.0, 1.0, 0.0),
        vel: tuple::vector(1.0, 1.0, 0.0).normalize(),
    };
    let env = Environment {
        gravity: tuple::vector(0.0, -0.1, 0.0),
        wind: tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut ticks = 0;

    while proj.pos.y > 0.0 {
        println!("Projectile pos: {:?}", proj.pos);
        proj = tick(&env, proj);
        ticks += 1;
    }
    println!("Ticks: {}", ticks);
}
