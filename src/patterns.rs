use crate::color::Color;
use crate::matrix::Mat4x4;
use crate::shape::Shape;
use crate::tuple::Tuple;

pub trait PatternTrait {
    fn color_at_object(&self, shape: &Shape, world_point: Tuple) -> Color;
}

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
}

impl PatternTrait for StripedPattern {
    fn color_at_object(&self, shape: &Shape, world_point: Tuple) -> Color {
        let object_point = shape.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;
        self.color_at(pattern_point)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GradientPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Mat4x4,
}

impl GradientPattern {
    pub fn new(a: Color, b: Color) -> Self {
        GradientPattern {
            a,
            b,
            transform: Mat4x4::identity(),
        }
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
    }
}

impl PatternTrait for GradientPattern {
    fn color_at_object(&self, shape: &Shape, world_point: Tuple) -> Color {
        let object_point = shape.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;
        self.color_at(pattern_point)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RingPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Mat4x4,
}

impl RingPattern {
    pub fn new(a: Color, b: Color) -> Self {
        RingPattern {
            a,
            b,
            transform: Mat4x4::identity(),
        }
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        let fac = (point.x * point.x + point.z * point.z).sqrt();
        if fac.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

impl PatternTrait for RingPattern {
    fn color_at_object(&self, shape: &Shape, world_point: Tuple) -> Color {
        let object_point = shape.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;
        self.color_at(pattern_point)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CheckerPattern {
    pub a: Color,
    pub b: Color,
    pub transform: Mat4x4,
}

impl CheckerPattern {
    pub fn new(a: Color, b: Color) -> Self {
        CheckerPattern {
            a,
            b,
            transform: Mat4x4::identity(),
        }
    }

    pub fn color_at(&self, point: Tuple) -> Color {
        let fac = point.x.floor() + point.y.floor() + point.z.floor();
        if fac % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

impl PatternTrait for CheckerPattern {
    fn color_at_object(&self, shape: &Shape, world_point: Tuple) -> Color {
        let object_point = shape.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;
        self.color_at(pattern_point)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Pattern {
    Stripe(StripedPattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Checker(CheckerPattern),
}

impl PatternTrait for Pattern {
    fn color_at_object(&self, shape: &Shape, world_point: Tuple) -> Color {
        match self {
            Pattern::Stripe(s) => s.color_at_object(shape, world_point),
            Pattern::Gradient(g) => g.color_at_object(shape, world_point),
            Pattern::Ring(r) => r.color_at_object(shape, world_point),
            Pattern::Checker(c) => c.color_at_object(shape, world_point),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::patterns::{
        CheckerPattern, GradientPattern, PatternTrait, RingPattern, StripedPattern,
    };
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

    #[test]
    fn gradient_pattern_linearly_interpolates_between_colors() {
        let pattern = GradientPattern::new(Color::white(), Color::black());
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 0.)));
        assert_eq!(
            Color::new(0.75, 0.75, 0.75),
            pattern.color_at(point(0.25, 0., 0.))
        );
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            pattern.color_at(point(0.5, 0., 0.))
        );
        assert_eq!(
            Color::new(0.25, 0.25, 0.25),
            pattern.color_at(point(0.75, 0., 0.))
        );
    }

    #[test]
    fn ring_pattern_should_extend_in_x_and_z() {
        let pattern = RingPattern::new(Color::white(), Color::black());
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 0.)));
        assert_eq!(Color::black(), pattern.color_at(point(1., 0., 0.)));
        assert_eq!(Color::black(), pattern.color_at(point(0., 0., 1.)));
        assert_eq!(Color::black(), pattern.color_at(point(0.708, 0., 0.708)));
    }

    #[test]
    fn checker_pattern_repeats_in_x() {
        let pattern = CheckerPattern::new(Color::white(), Color::black());
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 0.)));
        assert_eq!(Color::white(), pattern.color_at(point(0.99, 0., 0.)));
        assert_eq!(Color::black(), pattern.color_at(point(1.1, 0., 0.)));
    }

    #[test]
    fn checker_pattern_repeats_in_y() {
        let pattern = CheckerPattern::new(Color::white(), Color::black());
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 0.)));
        assert_eq!(Color::white(), pattern.color_at(point(0., 0.99, 0.)));
        assert_eq!(Color::black(), pattern.color_at(point(0., 1.1, 0.)));
    }

    #[test]
    fn checker_pattern_repeats_in_z() {
        let pattern = CheckerPattern::new(Color::white(), Color::black());
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 0.)));
        assert_eq!(Color::white(), pattern.color_at(point(0., 0., 0.99)));
        assert_eq!(Color::black(), pattern.color_at(point(0., 0., 1.1)));
    }
}
