use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(intensity: Color, position: Tuple) -> Self {
        PointLight {
            intensity,
            position,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::lights::PointLight;
    use crate::tuple::point;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1., 1., 1.);
        let position = point(0., 0., 0.);
        let light = PointLight::new(intensity, position);
        assert_eq!(position, light.position);
        assert_eq!(intensity, light.intensity);
    }
}
