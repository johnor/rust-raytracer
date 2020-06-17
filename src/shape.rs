use crate::tuple::Tuple;

pub trait Shape {
    fn normal(&self, p: Tuple) -> Tuple;
}
