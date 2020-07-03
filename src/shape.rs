use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrix::Mat4x4;
use crate::ray::Ray;
use crate::tuple::{point, Tuple};

pub type ShapeId = usize;

#[derive(Copy, Clone)]
pub enum Shape {
    Sphere,
}

#[derive(Copy, Clone)]
pub struct ShapeObject {
    pub shape: Shape,
    pub transform: Mat4x4,
    pub material: Material,
}

impl ShapeObject {
    pub fn new(shape: Shape) -> Self {
        ShapeObject {
            shape,
            transform: Mat4x4::identity(),
            material: Material::new(),
        }
    }
}

pub struct SurfaceNormalCalculator;

impl SurfaceNormalCalculator {
    pub fn calculate_normal(shape: Shape, transform: Mat4x4, wp: Tuple) -> Tuple {
        match shape {
            Shape::Sphere => Self::calculate_sphere_normal(transform, wp),
        }
    }

    fn calculate_sphere_normal(transform: Mat4x4, wp: Tuple) -> Tuple {
        let tinv = transform.inverse().unwrap();
        let on = (tinv * wp) - point(0., 0., 0.);
        let mut wn = tinv.transpose() * on;
        wn.w = 0.;
        wn.normalize()
    }
}

pub struct ShapeIntersectionHandler;

impl ShapeIntersectionHandler {
    pub fn intersect(
        shape_id: ShapeId,
        shape: Shape,
        transform: Mat4x4,
        ray: Ray,
    ) -> Vec<Intersection> {
        let ray2 = transform.inverse().unwrap() * ray;
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

        vec![
            Intersection::new(t1, shape_id),
            Intersection::new(t2, shape_id),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::materials::Material;
    use crate::matrix::Mat4x4;
    use crate::ray::Ray;
    use crate::shape::{Shape, ShapeIntersectionHandler, ShapeObject, SurfaceNormalCalculator};
    use crate::transform;
    use crate::tuple::test_utils::assert_tuple_eq;
    use crate::tuple::{point, vector};

    #[test]
    fn shape_default_transformation() {
        let s = ShapeObject::new(Shape::Sphere);
        assert_eq!(Mat4x4::identity(), s.transform);
    }

    #[test]
    fn shape_change_transformation() {
        let mut s = ShapeObject::new(Shape::Sphere);
        let t = transform::translate(2.0, 3.0, 4.0);
        s.transform = s.transform * t;
        assert_eq!(t, s.transform);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = ShapeObject::new(Shape::Sphere);
        assert_eq!(Material::new(), s.material);
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let mut s = ShapeObject::new(Shape::Sphere);
        let mut m = Material::new();
        m.ambient = 1.;
        s.material.ambient = m.ambient;
        assert_eq!(m, s.material);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = ShapeObject::new(Shape::Sphere);
        assert_tuple_eq(
            vector(1., 0., 0.),
            SurfaceNormalCalculator::calculate_normal(s.shape, s.transform, point(1., 0., 0.)),
        );
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = ShapeObject::new(Shape::Sphere);
        assert_tuple_eq(
            vector(0., 1., 0.),
            SurfaceNormalCalculator::calculate_normal(s.shape, s.transform, point(0., 1., 0.)),
        );
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = ShapeObject::new(Shape::Sphere);
        assert_tuple_eq(
            vector(0., 0., 1.),
            SurfaceNormalCalculator::calculate_normal(s.shape, s.transform, point(0., 0., 1.)),
        );
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let s = ShapeObject::new(Shape::Sphere);
        let v = 3_f64.sqrt() / 3.;
        assert_tuple_eq(
            vector(v, v, v),
            SurfaceNormalCalculator::calculate_normal(s.shape, s.transform, point(v, v, v)),
        );
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = ShapeObject::new(Shape::Sphere);
        s.transform = transform::translate(0., 1., 0.);
        assert_tuple_eq(
            vector(0., 0.70711, -0.70711),
            SurfaceNormalCalculator::calculate_normal(
                s.shape,
                s.transform,
                point(0., 1.70711, -0.70711),
            ),
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = ShapeObject::new(Shape::Sphere);
        s.transform =
            transform::scale(1., 0.5, 1.) * transform::rotate_z(std::f64::consts::PI / 5.);
        assert_tuple_eq(
            vector(0., 0.97014, -0.24254),
            SurfaceNormalCalculator::calculate_normal(
                s.shape,
                s.transform,
                point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.),
            ),
        );
    }

    #[test]
    fn ray_and_sphere_intersects_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(Shape::Sphere);
        let xs = ShapeIntersectionHandler::intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(Shape::Sphere);
        let xs = ShapeIntersectionHandler::intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(Shape::Sphere);
        let xs = ShapeIntersectionHandler::intersect(0, s.shape, s.transform, r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(Shape::Sphere);
        let xs = ShapeIntersectionHandler::intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(Shape::Sphere);
        let xs = ShapeIntersectionHandler::intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(Shape::Sphere);
        let xs = ShapeIntersectionHandler::intersect(0, s.shape, s.transform, r);
        assert_eq!(0, xs[0].shape_id);
        assert_eq!(0, xs[1].shape_id);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = ShapeObject::new(Shape::Sphere);
        s.transform = transform::scale(2.0, 2.0, 2.0);
        let xs = ShapeIntersectionHandler::intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = ShapeObject::new(Shape::Sphere);
        s.transform = transform::translate(5.0, 0.0, 0.0);
        let xs = ShapeIntersectionHandler::intersect(0, s.shape, s.transform, r);
        assert_eq!(0, xs.len());
    }
}
