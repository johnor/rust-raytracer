use crate::intersections::Intersection;
use crate::matrix::Mat4x4;
use crate::ray::Ray;
use crate::tuple::point;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: Mat4x4,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: Mat4x4::identity(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let ray2 = self.transform.inverse().unwrap() * ray;
        let sphere_to_ray = ray2.origin - point(0.0, 0.0, 0.0);
        let a = ray2.direction.dot(ray2.direction);
        let b = 2. * ray2.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![Intersection::new(t1, &self), Intersection::new(t2, &self)]
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Mat4x4;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::transform;
    use crate::tuple::{point, vector};

    #[test]
    fn ray_and_sphere_intersects_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersect(r);
        assert_eq!(&s, xs[0].object);
        assert_eq!(&s, xs[1].object);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();
        assert_eq!(Mat4x4::identity(), s.transform);
    }

    #[test]
    fn sphere_change_transformation() {
        let mut s = Sphere::new();
        let t = transform::translate(2.0, 3.0, 4.0);
        s.transform = t;
        assert_eq!(t, s.transform);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = transform::scale(2.0, 2.0, 2.0);

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = transform::translate(5.0, 0.0, 0.0);

        let xs = s.intersect(r);
        assert_eq!(0, xs.len());
    }
}
