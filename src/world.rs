use crate::color::Color;
use crate::intersections::Intersection;
use crate::lights::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transform::scale;
use crate::tuple::{point, Tuple};
use crate::shape::Shape;

pub struct World {
    pub light: PointLight,
    pub objects: Vec<Sphere>,
}

struct Comps<'a> {
    t: f64,
    object: &'a Sphere,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
}

impl World {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut xs = Vec::new();
        for obj in self.objects.iter() {
            xs.append(&mut obj.intersect(ray));
        }
        xs.sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
        xs
    }

    fn prepare_computations(intersection: Intersection, ray: Ray) -> Comps {
        let t = intersection.t;
        let object = intersection.object;
        let point = ray.position(intersection.t);
        let eyev =  -ray.direction;
        let normalv = object.normal(point);
        Comps{ t, object, point, eyev, normalv }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World {
            light: PointLight::new(Color::new(1., 1., 1.), point(-10., 10., -10.)),
            objects: Vec::new(),
        };

        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::new();
        s2.transform = s2.transform * scale(0.5, 0.5, 0.5);

        w.objects.push(s1);
        w.objects.push(s2);

        w
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::lights::PointLight;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::transform::scale;
    use crate::tuple::{point, vector};
    use crate::world::World;
    use crate::intersections::Intersection;

    #[test]
    fn creating_a_default_world() {
        let w = World::default();
        let expected_light = PointLight::new(Color::new(1., 1., 1.), point(-10., 10., -10.));

        let mut expected_obj1 = Sphere::new();
        expected_obj1.material.color = Color::new(0.8, 1.0, 0.6);
        expected_obj1.material.diffuse = 0.7;
        expected_obj1.material.specular = 0.2;

        let mut expected_obj2 = Sphere::new();
        expected_obj2.transform = expected_obj2.transform * scale(0.5, 0.5, 0.5);

        assert_eq!(expected_light, w.light);
        assert_eq!(expected_obj1, w.objects[0]);
        assert_eq!(expected_obj2, w.objects[1]);
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
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = Sphere::new();
        let i = Intersection::new(4., &s);
        let c = World::prepare_computations(i, r);
        assert_eq!(c.t, i.t);
        assert_eq!(*c.object, s);
        assert_eq!(c.point, point(0., 0., -1.));
        assert_eq!(c.eyev, vector(0., 0., -1.));
        assert_eq!(c.normalv, vector(0., 0., -1.));
    }
}
