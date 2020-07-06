use crate::color::Color;
use crate::matrix::Mat4x4;
use crate::shape::Shape;
use crate::tuple::Tuple;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StripedPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Mat4x4,
}

impl StripedPattern {
    pub fn new(a: Color, b: Color) -> Self {
        StripedPattern {
            a,
            b,
            transform: Mat4x4::identity(),
        }
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        if point.x.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
    pub fn color_at_object(&self, object: &Shape, world_point: Tuple) -> Color {
        let object_point = object.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;
        self.color_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::patterns::StripedPattern;
    use crate::shape::{Shape, ShapeType};
    use crate::transform;
    use crate::tuple::point;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = StripedPattern::new(Color::white(), Color::black());
        assert_eq!(Color::white(), pattern.a);
        assert_eq!(Color::black(), pattern.b);
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let pattern = StripedPattern::new(Color::white(), Color::black());
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 0.)));
        assert_eq!(Color::white(), pattern.color_at(point(0., 1., 0.)));
        assert_eq!(Color::white(), pattern.color_at(point(0., 2., 0.)));
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let pattern = StripedPattern::new(Color::white(), Color::black());
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 0.)));
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 1.)));
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 2.)));
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = StripedPattern::new(Color::white(), Color::black());
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 0.)));
        assert_eq!(Color::white(), pattern.color_at(point(0.9, 0., 0.)));
        assert_eq!(Color::black(), pattern.color_at(point(1., 0., 0.)));
        assert_eq!(Color::black(), pattern.color_at(point(-0.1, 0., 0.)));
        assert_eq!(Color::black(), pattern.color_at(point(-1., 0., 0.)));
        assert_eq!(Color::white(), pattern.color_at(point(-1.1, 0., 0.)));
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Shape::new(ShapeType::Sphere);
        object.transform = transform::scale(2., 2., 2.);
        let pattern = StripedPattern::new(Color::white(), Color::black());
        let c = pattern.color_at_object(&object, point(1.5, 0., 0.));
        assert_eq!(Color::white(), c);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Shape::new(ShapeType::Sphere);
        let mut pattern = StripedPattern::new(Color::white(), Color::black());
        pattern.transform = transform::scale(2., 2., 2.);
        let c = pattern.color_at_object(&object, point(1.5, 0., 0.));
        assert_eq!(Color::white(), c);
    }

    #[test]
    fn stripes_with_bot_object_and_pattern_transformation() {
        let mut object = Shape::new(ShapeType::Sphere);
        object.transform = transform::scale(2., 2., 2.);
        let mut pattern = StripedPattern::new(Color::white(), Color::black());
        pattern.transform = transform::translate(0.5, 0., 0.);
        let c = pattern.color_at_object(&object, point(2.5, 0., 0.));
        assert_eq!(Color::white(), c);
    }
}
