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
    use crate::ray::Ray;
    use crate::shape::{glass_sphere, Shape, ShapeType};
    use crate::transform;
    use crate::tuple::point;
    use crate::tuple::vector;
    use crate::world::World;

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

    macro_rules! find_n1_n2_test {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (index, n1, n2) = $value;
                    let mut a = glass_sphere();
                    a.transform = transform::scale(2., 2., 2.);
                    a.material.refractive_index = 1.5;

                    let mut b = glass_sphere();
                    b.transform = transform::translate(0., 0., -0.25);
                    b.material.refractive_index = 2.;

                    let mut c = glass_sphere();
                    c.transform = transform::translate(0., 0., 0.25);
                    c.material.refractive_index = 2.5;

                    let r = Ray::new(point(0., 0., -4.), vector(0., 0., 1.));
                    let xs = vec![
                        Intersection::new(2., &a),
                        Intersection::new(2.75, &b),
                        Intersection::new(3.25, &c),
                        Intersection::new(4.75, &b),
                        Intersection::new(5.25, &c),
                        Intersection::new(6., &a)
                    ];
                    let comps = World::prepare_computations_with_intersections(xs[index], r, xs);
                    assert_eq!(n1, comps.n1);
                    assert_eq!(n2, comps.n2);
                }
            )*
        }
    }

    find_n1_n2_test! {
        prepare_computations_finds_n1_n2_for_index_0: (0, 1., 1.5),
        prepare_computations_finds_n1_n2_for_index_1: (1, 1.5, 2.),
        prepare_computations_finds_n1_n2_for_index_2: (2, 2., 2.5),
        prepare_computations_finds_n1_n2_for_index_3: (3, 2.5, 2.5),
        prepare_computations_finds_n1_n2_for_index_4: (4, 2.5, 1.5),
        prepare_computations_finds_n1_n2_for_index_5: (5, 1.5, 1.),
    }
}
