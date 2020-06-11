use crate::ray::Ray;
use crate::tuple::{point, Tuple};

#[derive(Debug)]
pub struct Sphere {
    pub origin: Tuple,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            origin: point(0.0, 0.0, 0.0),
        }
    }

    pub fn intersects(&self, ray: Ray) -> Vec<f64> {
        let sphere_to_ray = ray.origin - self.origin;
        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        vec![t1, t2]
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::point;
    use crate::tuple::vector;

    #[test]
    fn ray_and_sphere_intersects_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0]);
        assert_eq!(6.0, xs[1]);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0]);
        assert_eq!(5.0, xs[1]);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0]);
        assert_eq!(1.0, xs[1]);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();

        let xs = s.intersects(r);
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0]);
        assert_eq!(-4.0, xs[1]);
    }
}
