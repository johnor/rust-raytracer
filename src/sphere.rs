use crate::intersections::Intersection;
use crate::matrix::Mat4x4;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::{point, Tuple};

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

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape for Sphere {
    fn normal(&self, wp: Tuple) -> Tuple {
        let tinv = self.transform.inverse().unwrap();
        let on = (tinv * wp) - point(0., 0., 0.);
        let mut wn = tinv.transpose() * on;
        wn.w = 0.;
        wn.normalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Mat4x4;
    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::sphere::Sphere;
    use crate::transform;
    use crate::transform::translate;
    use crate::tuple::test_utils::assert_tuple_eq;
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

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        assert_tuple_eq(vector(1., 0., 0.), s.normal(point(1., 0., 0.)));
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        assert_tuple_eq(vector(0., 1., 0.), s.normal(point(0., 1., 0.)));
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        assert_tuple_eq(vector(0., 0., 1.), s.normal(point(0., 0., 1.)));
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let v = 3_f64.sqrt() / 3.;
        assert_tuple_eq(vector(v, v, v), s.normal(point(v, v, v)));
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.transform = s.transform * transform::translate(0., 1., 0.);
        assert_tuple_eq(
            vector(0., 0.70711, -0.70711),
            s.normal(point(0., 1.70711, -0.70711)),
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        let m = transform::scale(1., 0.5, 1.) * transform::rotate_z(std::f64::consts::PI / 5.);
        s.transform = s.transform * m;
        assert_tuple_eq(
            vector(0., 0.97014, -0.24254),
            s.normal(point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.)),
        );
    }
}
