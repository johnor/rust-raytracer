use crate::color::Color;
use crate::intersections::{hit, Intersection};
use crate::lights::PointLight;
use crate::materials::Material;
use crate::ray::Ray;
use crate::shape::{ShapeHandler, ShapeId, ShapeIntersectionHandler, SurfaceNormalCalculator};
use crate::transform::scale;
use crate::tuple::{point, Tuple};

pub struct World {
    pub light: PointLight,
    pub shape_handler: ShapeHandler,
}

struct Comps {
    t: f64,
    shape_id: ShapeId,
    point: Tuple,
    over_point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    inside: bool,
}

impl Comps {
    const OVER_POINT_EPSILON: f64 = 0.000_000_1;
}

impl World {
    pub fn new() -> Self {
        World {
            light: PointLight::new(Color::new(1., 1., 1.), point(-10., 10., -10.)),
            shape_handler: ShapeHandler::new(),
        }
    }
    pub fn color_at(&self, ray: Ray) -> Color {
        match hit(self.intersect(ray)) {
            Some(i) => self.shade_hit(self.prepare_computations(i, ray)),
            None => Color::new(0., 0., 0.),
        }
    }

    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut xs = Vec::new();
        for id in 0..self.shape_handler.number_of_shapes() {
            xs.append(&mut ShapeIntersectionHandler::intersect(
                self.shape_handler.info(id),
                self.shape_handler.transform(id),
                ray,
            ));
        }
        xs.sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
        xs
    }

    fn prepare_computations(&self, intersection: Intersection, ray: Ray) -> Comps {
        let t = intersection.t;
        let shape_id = intersection.shape_id;
        let point = ray.position(intersection.t);
        let eyev = -ray.direction;
        let mut normalv = SurfaceNormalCalculator::calculate_normal(
            self.shape_handler.info(shape_id).0,
            self.shape_handler.transform(shape_id),
            point,
        );
        let over_point = point + normalv * Comps::OVER_POINT_EPSILON;
        let inside = if normalv.dot(eyev) < 0. {
            normalv = -normalv;
            true
        } else {
            false
        };
        Comps {
            t,
            shape_id,
            point,
            over_point,
            eyev,
            normalv,
            inside,
        }
    }

    fn shade_hit(&self, comps: Comps) -> Color {
        Material::lighting(
            self.shape_handler.material(comps.shape_id),
            self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(comps.over_point),
        )
    }

    fn is_shadowed(&self, p: Tuple) -> bool {
        let direction = self.light.position - p;
        let distance = direction.magnitude();
        let ray = Ray::new(p, direction.normalize());
        match hit(self.intersect(ray)) {
            Some(i) => i.t < distance,
            None => false,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World {
            light: PointLight::new(Color::new(1., 1., 1.), point(-10., 10., -10.)),
            shape_handler: ShapeHandler::new(),
        };

        let s1 = w.shape_handler.create_sphere();
        let m1 = w.shape_handler.material_mut(s1);
        m1.color = Color::new(0.8, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;

        let s2 = w.shape_handler.create_sphere();
        w.shape_handler.add_transform(s2, scale(0.5, 0.5, 0.5));

        w
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::intersections::Intersection;
    use crate::lights::PointLight;
    use crate::materials::Material;
    use crate::matrix::Mat4x4;
    use crate::ray::Ray;
    use crate::test_utils::assert_color_near;
    use crate::transform::{scale, translate};
    use crate::tuple::{point, vector};
    use crate::world::{Comps, World};

    #[test]
    fn creating_a_default_world() {
        let w = World::default();
        let expected_light = PointLight::new(Color::new(1., 1., 1.), point(-10., 10., -10.));

        let mut expected_material = Material::new();
        expected_material.color = Color::new(0.8, 1.0, 0.6);
        expected_material.diffuse = 0.7;
        expected_material.specular = 0.2;

        let mut expected_transform = Mat4x4::identity();
        expected_transform = expected_transform * scale(0.5, 0.5, 0.5);

        assert_eq!(expected_light, w.light);
        assert_eq!(Mat4x4::identity(), w.shape_handler.transform(0));
        assert_eq!(expected_material, w.shape_handler.material(0));
        assert_eq!(expected_transform, w.shape_handler.transform(1));
        assert_eq!(Material::new(), w.shape_handler.material(1));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = w.intersect(r);
        assert_eq!(4, xs.len());
        assert_eq!(4., xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6., xs[3].t);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let mut w = World::new();
        let s = w.shape_handler.create_sphere();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let i = Intersection::new(4., s);
        let c = w.prepare_computations(i, r);
        assert_eq!(c.t, i.t);
        assert_eq!(c.shape_id, s);
        assert_eq!(c.point, point(0., 0., -1.));
        assert_eq!(c.eyev, vector(0., 0., -1.));
        assert_eq!(c.normalv, vector(0., 0., -1.));
    }

    #[test]
    fn hit_when_interserction_occurs_on_the_outside() {
        let mut w = World::new();
        let s = w.shape_handler.create_sphere();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let i = Intersection::new(4., s);
        let c = w.prepare_computations(i, r);
        assert_eq!(c.inside, false);
    }

    #[test]
    fn hit_when_interserction_occurs_on_the_inside() {
        let mut w = World::new();
        let s = w.shape_handler.create_sphere();
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let i = Intersection::new(1., s);
        let c = w.prepare_computations(i, r);
        assert_eq!(c.point, point(0., 0., 1.));
        assert_eq!(c.eyev, vector(0., 0., -1.));
        assert_eq!(c.inside, true);
        assert_eq!(c.normalv, vector(0., 0., -1.));
    }

    #[test]
    fn hit_should_offset_the_point() {
        let mut w = World::new();
        let s = w.shape_handler.create_sphere();
        w.shape_handler.add_transform(s, translate(0., 0., 1.));
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let i = Intersection::new(5., s);
        let comps = w.prepare_computations(i, r);
        assert!(comps.over_point.z < -Comps::OVER_POINT_EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let i = Intersection::new(4., 0);
        let com = w.prepare_computations(i, r);
        let col = w.shade_hit(com);
        assert_color_near(col, Color::new(0.38066, 0.47583, 0.2855), 0.0001);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = PointLight::new(Color::new(1., 1., 1.), point(0., 0.25, 0.));
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let i = Intersection::new(0.5, 1);
        let com = w.prepare_computations(i, r);
        let col = w.shade_hit(com);
        assert_eq!(col, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn interection_in_shadow() {
        let mut w = World::default();
        w.light = PointLight::new(Color::white(), point(0., 0., -10.));
        w.shape_handler.add_transform(1, translate(0., 0., 10.));
        let r = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
        let i = Intersection::new(4., 1);
        let comps = w.prepare_computations(i, r);
        assert_eq!(Color::new(0.1, 0.1, 0.1), w.shade_hit(comps));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 1., 0.));
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let c = w.color_at(r);
        assert_color_near(c, Color::new(0.38066, 0.47583, 0.2855), 0.0001);
    }

    #[test]
    fn the_color_when_an_intersection_behind_the_ray() {
        let w = World::default();
        w.shape_handler.material(0).ambient = 1.;
        w.shape_handler.material(1).ambient = 1.;
        let r = Ray::new(point(0., 0., 0.75), vector(0., 0., -1.));
        let c = w.color_at(r);
        assert_color_near(c, w.shape_handler.material(1).color, 0.0001);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        assert_eq!(false, w.is_shadowed(point(0., 10., 0.)));
    }

    #[test]
    fn shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        assert_eq!(true, w.is_shadowed(point(10., -10., 10.)));
    }

    #[test]
    fn no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        assert_eq!(false, w.is_shadowed(point(-20., 20., -20.)));
    }

    #[test]
    fn no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        assert_eq!(false, w.is_shadowed(point(-2., 2., -2.)));
    }
}
