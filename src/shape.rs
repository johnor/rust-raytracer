use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrix::Mat4x4;
use crate::ray::Ray;
use crate::tuple::{point, Tuple};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShapeType {
    Sphere,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shape {
    pub shape: ShapeType,
    pub transform: Mat4x4,
    pub material: Material,
}

impl Shape {
    pub fn new(shape: ShapeType) -> Self {
        Shape {
            shape,
            transform: Mat4x4::identity(),
            material: Material::new(),
        }
    }

    pub fn normal(&self, p: Tuple) -> Tuple {
        let tinv = self.transform.inverse().unwrap();
        let local_point = tinv * p;
        let local_normal = match self.shape {
            ShapeType::Sphere => calculate_sphere_normal(local_point),
        };
        let mut world_normal = tinv.transpose() * local_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = self.transform.inverse().unwrap() * ray;
        match self.shape {
            ShapeType::Sphere => intersect_sphere(&self, local_ray),
        }
    }
}

fn calculate_sphere_normal(p: Tuple) -> Tuple {
    p - point(0., 0., 0.)
}

fn intersect_sphere(shape: &Shape, ray: Ray) -> Vec<Intersection> {
    let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);
    let a = ray.direction.dot(ray.direction);
    let b = 2. * ray.direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return Vec::new();
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    vec![Intersection::new(t1, shape), Intersection::new(t2, shape)]
}

#[cfg(test)]
mod tests {
    use crate::materials::Material;
    use crate::matrix::Mat4x4;
    use crate::ray::Ray;
    use crate::shape::{Shape, ShapeType};
    use crate::transform;
    use crate::tuple::test_utils::assert_tuple_eq;
    use crate::tuple::{point, vector};

    #[test]
    fn shape_default_transformation() {
        let s = Shape::new(ShapeType::Sphere);
        assert_eq!(Mat4x4::identity(), s.transform);
    }

    #[test]
    fn shape_change_transformation() {
        let mut s = Shape::new(ShapeType::Sphere);
        let t = transform::translate(2.0, 3.0, 4.0);
        s.transform = s.transform * t;
        assert_eq!(t, s.transform);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Shape::new(ShapeType::Sphere);
        assert_eq!(Material::new(), s.material);
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let mut s = Shape::new(ShapeType::Sphere);
        let mut m = Material::new();
        m.ambient = 1.;
        s.material.ambient = m.ambient;
        assert_eq!(m, s.material);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Shape::new(ShapeType::Sphere);
        assert_tuple_eq(vector(1., 0., 0.), s.normal(point(1., 0., 0.)));
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Shape::new(ShapeType::Sphere);
        assert_tuple_eq(vector(0., 1., 0.), s.normal(point(0., 1., 0.)));
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Shape::new(ShapeType::Sphere);
        assert_tuple_eq(vector(0., 0., 1.), s.normal(point(0., 0., 1.)));
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Shape::new(ShapeType::Sphere);
        let v = 3_f64.sqrt() / 3.;
        assert_tuple_eq(vector(v, v, v), s.normal(point(v, v, v)));
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Shape::new(ShapeType::Sphere);
        s.transform = transform::translate(0., 1., 0.);
        assert_tuple_eq(
            vector(0., 0.70711, -0.70711),
            s.normal(point(0., 1.70711, -0.70711)),
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Shape::new(ShapeType::Sphere);
        s.transform =
            transform::scale(1., 0.5, 1.) * transform::rotate_z(std::f64::consts::PI / 5.);
        assert_tuple_eq(
            vector(0., 0.97014, -0.24254),
            s.normal(point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.)),
        );
    }

    #[test]
    fn ray_and_sphere_intersects_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::new(ShapeType::Sphere);
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::new(ShapeType::Sphere);
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::new(ShapeType::Sphere);
        let xs = s.intersect(r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Shape::new(ShapeType::Sphere);
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::new(ShapeType::Sphere);
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Shape::new(ShapeType::Sphere);
        let xs = s.intersect(r);
        assert_eq!(&s, xs[0].shape);
        assert_eq!(&s, xs[1].shape);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Shape::new(ShapeType::Sphere);
        s.transform = transform::scale(2.0, 2.0, 2.0);
        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = Shape::new(ShapeType::Sphere);
        s.transform = transform::translate(5.0, 0.0, 0.0);
        let xs = s.intersect(r);
        assert_eq!(0, xs.len());
    }
}
