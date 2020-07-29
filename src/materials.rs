use crate::color::Color;
use crate::lights::PointLight;
use crate::patterns::{Pattern, PatternTrait};
use crate::shape::Shape;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
            reflective: 0.,
            transparency: 0.,
            refractive_index: 1.,
            pattern: None,
        }
    }

    pub fn lighting(
        material: Material,
        object: &Shape,
        light: PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = match material.pattern {
            Some(pattern) => pattern.color_at_object(object, point),
            None => material.color,
        };
        let effective_color = color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * material.ambient;

        let mut diffuse = Color::black();
        let mut specular = Color::black();
        if !in_shadow {
            let light_dot_normal = lightv.dot(normalv);
            if light_dot_normal >= 0. {
                diffuse = effective_color * material.diffuse * light_dot_normal;

                let reflectv = (-lightv).reflect(normalv);
                let reflect_dot_eye = reflectv.dot(eyev);

                if reflect_dot_eye > 0. {
                    let factor = reflect_dot_eye.powf(material.shininess);
                    specular = light.intensity * material.specular * factor;
                }
            }
        }

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::lights::PointLight;
    use crate::materials::Material;
    use crate::patterns::{Pattern, StripedPattern};
    use crate::shape::{Shape, ShapeType};
    use crate::test_utils::assert_color_near;
    use crate::tuple::{point, vector};

    #[test]
    fn defaut_material() {
        let m = Material::default();
        assert_eq!(Color::new(1., 1., 1.), m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200., m.shininess);
        assert_eq!(0., m.reflective);
        assert_eq!(0., m.transparency);
        assert_eq!(1., m.refractive_index);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 0., -10.));
        let object = Shape::new(ShapeType::Sphere);
        let result = Material::lighting(m, &object, light, position, eyev, normalv, false);
        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_deg() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 0., -10.));
        let object = Shape::new(ShapeType::Sphere);
        let result = Material::lighting(m, &object, light, position, eyev, normalv, false);
        assert_eq!(Color::new(1.0, 1.0, 1.0), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 10., -10.));
        let object = Shape::new(ShapeType::Sphere);
        let result = Material::lighting(m, &object, light, position, eyev, normalv, false);
        assert_color_near(Color::new(0.7364, 0.7364, 0.7364), result, 0.00001);
    }

    #[test]
    fn lighting_with_eye_in_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., -2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 10., -10.));
        let object = Shape::new(ShapeType::Sphere);
        let result = Material::lighting(m, &object, light, position, eyev, normalv, false);
        assert_color_near(Color::new(1.6364, 1.6364, 1.6364), result, 0.00001);
    }

    #[test]
    fn lighting_light_behind_surface() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 0., 10.));
        let object = Shape::new(ShapeType::Sphere);
        let result = Material::lighting(m, &object, light, position, eyev, normalv, false);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 0., -10.));
        let object = Shape::new(ShapeType::Sphere);
        let result = Material::lighting(m, &object, light, position, eyev, normalv, true);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let mut m = Material::new();
        m.pattern = Some(Pattern::Stripe(StripedPattern::new(
            Color::white(),
            Color::black(),
        )));
        m.ambient = 1.;
        m.diffuse = 0.;
        m.specular = 0.;

        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 0., -10.));
        let object = Shape::new(ShapeType::Sphere);
        let c1 = Material::lighting(m, &object, light, point(0.9, 0., 0.), eyev, normalv, false);
        let c2 = Material::lighting(m, &object, light, point(1.1, 0., 0.), eyev, normalv, false);
        assert_eq!(Color::white(), c1);
        assert_eq!(Color::black(), c2);
    }
}
