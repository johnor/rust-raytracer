use crate::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::materials::Material;

    #[test]
    fn defaut_material() {
        let m = Material::new();
        assert_eq!(Color::new(1., 1., 1.), m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200., m.shininess);
    }
}
