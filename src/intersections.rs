use crate::shape::Shape;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub shape: &'a Shape,
}

impl Intersection<'_> {
    pub fn new(t: f64, shape: &Shape) -> Intersection {
        Intersection { t, shape }
    }
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let min_intersection = intersections
        .iter()
        .filter(|i| i.t >= 0.0)
        .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap());

    min_intersection.cloned()
}

#[cfg(test)]
mod tests {
    use crate::intersections::{hit, Intersection};
    use crate::shape::{Shape, ShapeType};

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Shape::new(ShapeType::Sphere);
        let i = Intersection::new(3.5, &s);

        assert_eq!(3.5, i.t);
        assert_eq!(&s, i.shape);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Shape::new(ShapeType::Sphere);
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];

        assert_eq!(i1, hit(xs).unwrap());
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Shape::new(ShapeType::Sphere);
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];

        assert_eq!(i2, hit(xs).unwrap());
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Shape::new(ShapeType::Sphere);
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(-2.0, &s);
        let xs = vec![i1, i2];

        assert_eq!(None, hit(xs));
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let s = Shape::new(ShapeType::Sphere);
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];

        assert_eq!(i4, hit(xs).unwrap());
    }
}
