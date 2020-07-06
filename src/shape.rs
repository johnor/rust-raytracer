use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrix::Mat4x4;
use crate::ray::Ray;
use crate::tuple::{point, Tuple};

pub type ShapeId = usize;

#[derive(Copy, Clone)]
pub enum ShapeType {
    Sphere,
}

#[derive(Copy, Clone)]
pub struct ShapeObject {
    pub shape: ShapeType,
    pub transform: Mat4x4,
    pub material: Material,
}

impl ShapeObject {
    pub fn new(shape: ShapeType) -> Self {
        ShapeObject {
            shape,
            transform: Mat4x4::identity(),
            material: Material::new(),
        }
    }
}

pub fn calculate_normal(shape: ShapeType, transform: Mat4x4, point: Tuple) -> Tuple {
    let tinv = transform.inverse().unwrap();
    let local_point  =  tinv * point;
    let local_normal = match shape {
        ShapeType::Sphere => calculate_sphere_normal(local_point),
    };
    let mut world_normal = tinv.transpose() * local_normal;
    world_normal.w = 0.;
    world_normal.normalize()
}

fn calculate_sphere_normal(p: Tuple) -> Tuple {
    p - point(0., 0., 0.)
}

pub fn intersect(
    shape_id: ShapeId,
    shape: ShapeType,
    transform: Mat4x4,
    ray: Ray,
) -> Vec<Intersection> {
    let local_ray = transform.inverse().unwrap() * ray;
    match shape {
        ShapeType::Sphere => intersect_sphere(shape_id, local_ray),
    }
}

fn intersect_sphere(shape_id: ShapeId, ray: Ray) -> Vec<Intersection> {
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

    vec![
        Intersection::new(t1, shape_id),
        Intersection::new(t2, shape_id),
    ]
}

#[cfg(test)]
mod tests {
    use crate::materials::Material;
    use crate::matrix::Mat4x4;
    use crate::ray::Ray;
    use crate::shape::{calculate_normal, intersect, ShapeType, ShapeObject};
    use crate::transform;
    use crate::tuple::test_utils::assert_tuple_eq;
    use crate::tuple::{point, vector};

    #[test]
    fn shape_default_transformation() {
        let s = ShapeObject::new(ShapeType::Sphere);
        assert_eq!(Mat4x4::identity(), s.transform);
    }

    #[test]
    fn shape_change_transformation() {
        let mut s = ShapeObject::new(ShapeType::Sphere);
        let t = transform::translate(2.0, 3.0, 4.0);
        s.transform = s.transform * t;
        assert_eq!(t, s.transform);
    }

    #[test]
    fn sphere_has_default_material() {
        let s = ShapeObject::new(ShapeType::Sphere);
        assert_eq!(Material::new(), s.material);
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let mut s = ShapeObject::new(ShapeType::Sphere);
        let mut m = Material::new();
        m.ambient = 1.;
        s.material.ambient = m.ambient;
        assert_eq!(m, s.material);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = ShapeObject::new(ShapeType::Sphere);
        assert_tuple_eq(
            vector(1., 0., 0.),
            calculate_normal(s.shape, s.transform, point(1., 0., 0.)),
        );
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = ShapeObject::new(ShapeType::Sphere);
        assert_tuple_eq(
            vector(0., 1., 0.),
            calculate_normal(s.shape, s.transform, point(0., 1., 0.)),
        );
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = ShapeObject::new(ShapeType::Sphere);
        assert_tuple_eq(
            vector(0., 0., 1.),
            calculate_normal(s.shape, s.transform, point(0., 0., 1.)),
        );
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let s = ShapeObject::new(ShapeType::Sphere);
        let v = 3_f64.sqrt() / 3.;
        assert_tuple_eq(
            vector(v, v, v),
            calculate_normal(s.shape, s.transform, point(v, v, v)),
        );
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = ShapeObject::new(ShapeType::Sphere);
        s.transform = transform::translate(0., 1., 0.);
        assert_tuple_eq(
            vector(0., 0.70711, -0.70711),
            calculate_normal(s.shape, s.transform, point(0., 1.70711, -0.70711)),
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = ShapeObject::new(ShapeType::Sphere);
        s.transform =
            transform::scale(1., 0.5, 1.) * transform::rotate_z(std::f64::consts::PI / 5.);
        assert_tuple_eq(
            vector(0., 0.97014, -0.24254),
            calculate_normal(
                s.shape,
                s.transform,
                point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.),
            ),
        );
    }

    #[test]
    fn ray_and_sphere_intersects_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(ShapeType::Sphere);
        let xs = intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(ShapeType::Sphere);
        let xs = intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(ShapeType::Sphere);
        let xs = intersect(0, s.shape, s.transform, r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(ShapeType::Sphere);
        let xs = intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(ShapeType::Sphere);
        let xs = intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = ShapeObject::new(ShapeType::Sphere);
        let xs = intersect(0, s.shape, s.transform, r);
        assert_eq!(0, xs[0].shape_id);
        assert_eq!(0, xs[1].shape_id);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = ShapeObject::new(ShapeType::Sphere);
        s.transform = transform::scale(2.0, 2.0, 2.0);
        let xs = intersect(0, s.shape, s.transform, r);
        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = ShapeObject::new(ShapeType::Sphere);
        s.transform = transform::translate(5.0, 0.0, 0.0);
        let xs = intersect(0, s.shape, s.transform, r);
        assert_eq!(0, xs.len());
    }
}
