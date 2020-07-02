use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrix::Mat4x4;
use crate::ray::Ray;
use crate::tuple::{point, Tuple};
use std::borrow::BorrowMut;

#[derive(Copy, Clone)]
pub enum ShapeType {
    Sphere,
}

pub type ShapeId = usize;

pub type ShapeInfo = (ShapeType, ShapeId);

pub struct ShapeHandler {
    types: Vec<ShapeType>,
    transforms: Vec<Mat4x4>,
    materials: Vec<Material>,
}

impl ShapeHandler {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            transforms: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn number_of_shapes(&self) -> usize {
        self.types.len()
    }

    pub fn create_sphere(&mut self) -> ShapeId {
        self.create_shape(ShapeType::Sphere)
    }

    fn create_shape(&mut self, ty: ShapeType) -> ShapeId {
        self.types.push(ty);
        self.transforms.push(Mat4x4::identity());
        self.materials.push(Material::new());
        (self.types.len() as ShapeId) - 1
    }

    pub fn info(&self, id: ShapeId) -> ShapeInfo {
        (self.types[id], id)
    }

    pub fn transform(&self, id: ShapeId) -> Mat4x4 {
        self.transforms[id]
    }

    pub fn add_transform(&mut self, id: ShapeId, transform: Mat4x4) {
        self.transforms[id] = self.transforms[id] * transform;
    }

    pub fn material(&self, id: ShapeId) -> Material {
        self.materials[id]
    }

    pub fn material_mut(&mut self, id: ShapeId) -> &mut Material {
        self.materials[id].borrow_mut()
    }

    pub fn set_material(&mut self, id: ShapeId, material: Material) {
        self.materials[id] = material;
    }
}

pub struct SurfaceNormalCalculator;

impl SurfaceNormalCalculator {
    pub fn calculate_normal(ty: ShapeType, transform: Mat4x4, wp: Tuple) -> Tuple {
        match ty {
            ShapeType::Sphere => SurfaceNormalCalculator::calculate_sphere_normal(transform, wp),
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
    pub fn intersect(shape_info: ShapeInfo, transform: Mat4x4, ray: Ray) -> Vec<Intersection> {
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
            Intersection::new(t1, shape_info.1),
            Intersection::new(t2, shape_info.1),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::materials::Material;
    use crate::matrix::Mat4x4;
    use crate::ray::Ray;
    use crate::shape::{ShapeHandler, ShapeIntersectionHandler, SurfaceNormalCalculator};
    use crate::transform;
    use crate::tuple::test_utils::assert_tuple_eq;
    use crate::tuple::{point, vector};

    #[test]
    fn shape_default_transformation() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();
        assert_eq!(Mat4x4::identity(), sh.transform(id));
    }

    #[test]
    fn shape_change_transformation() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();
        let t = transform::translate(2.0, 3.0, 4.0);
        sh.add_transform(id, t);
        assert_eq!(t, sh.transform(id));
    }

    #[test]
    fn sphere_has_default_material() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();
        assert_eq!(Material::new(), sh.material(id));
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();

        let mut m = Material::new();
        m.ambient = 1.;

        let sm = sh.material_mut(id);
        sm.ambient = m.ambient;

        assert_eq!(m, sh.material(id));
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();
        assert_tuple_eq(
            vector(1., 0., 0.),
            SurfaceNormalCalculator::calculate_normal(
                sh.info(id).0,
                sh.transform(id),
                point(1., 0., 0.),
            ),
        );
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();
        assert_tuple_eq(
            vector(0., 1., 0.),
            SurfaceNormalCalculator::calculate_normal(
                sh.info(id).0,
                sh.transform(id),
                point(0., 1., 0.),
            ),
        );
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();
        assert_tuple_eq(
            vector(0., 0., 1.),
            SurfaceNormalCalculator::calculate_normal(
                sh.info(id).0,
                sh.transform(id),
                point(0., 0., 1.),
            ),
        );
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();
        let v = 3_f64.sqrt() / 3.;
        assert_tuple_eq(
            vector(v, v, v),
            SurfaceNormalCalculator::calculate_normal(
                sh.info(id).0,
                sh.transform(id),
                point(v, v, v),
            ),
        );
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();
        sh.add_transform(id, transform::translate(0., 1., 0.));
        assert_tuple_eq(
            vector(0., 0.70711, -0.70711),
            SurfaceNormalCalculator::calculate_normal(
                sh.info(id).0,
                sh.transform(id),
                point(0., 1.70711, -0.70711),
            ),
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();
        let m = transform::scale(1., 0.5, 1.) * transform::rotate_z(std::f64::consts::PI / 5.);
        sh.add_transform(id, m);
        assert_tuple_eq(
            vector(0., 0.97014, -0.24254),
            SurfaceNormalCalculator::calculate_normal(
                sh.info(id).0,
                sh.transform(id),
                point(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.),
            ),
        );
    }

    #[test]
    fn ray_and_sphere_intersects_at_two_points() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();

        let xs = ShapeIntersectionHandler::intersect(sh.info(id), sh.transform(id), r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();

        let xs = ShapeIntersectionHandler::intersect(sh.info(id), sh.transform(id), r);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();

        let xs = ShapeIntersectionHandler::intersect(sh.info(id), sh.transform(id), r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();

        let xs = ShapeIntersectionHandler::intersect(sh.info(id), sh.transform(id), r);
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();

        let xs = ShapeIntersectionHandler::intersect(sh.info(id), sh.transform(id), r);
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn intersect_sets_object_on_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();

        let xs = ShapeIntersectionHandler::intersect(sh.info(id), sh.transform(id), r);
        assert_eq!(id, xs[0].shape_id);
        assert_eq!(id, xs[1].shape_id);
    }

    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();

        sh.add_transform(id, transform::scale(2.0, 2.0, 2.0));

        let xs = ShapeIntersectionHandler::intersect(sh.info(id), sh.transform(id), r);
        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut sh = ShapeHandler::new();
        let id = sh.create_sphere();

        sh.add_transform(id, transform::translate(5.0, 0.0, 0.0));

        let xs = ShapeIntersectionHandler::intersect(sh.info(id), sh.transform(id), r);
        assert_eq!(0, xs.len());
    }
}
