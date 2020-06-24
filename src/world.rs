use crate::color::Color;
use crate::lights::PointLight;
use crate::sphere::Sphere;
use crate::transform::scale;
use crate::tuple::point;

pub struct World {
    pub light: PointLight,
    pub objects: Vec<Sphere>,
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
    use crate::sphere::Sphere;
    use crate::transform::scale;
    use crate::tuple::point;
    use crate::world::World;

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
}
