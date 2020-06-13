use crate::sphere::Sphere;

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl Intersection<'_> {
    pub fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection { t, object: &object }
    }
}

#[cfg(test)]
mod tests {
    use crate::intersections::Intersection;
    use crate::sphere::Sphere;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);

        assert_eq!(3.5, i.t);
        assert_eq!(&s, i.object);
    }
}
