mod tuple;

use tuple::Tuple;

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
}
