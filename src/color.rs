use std::ops::{Add, Sub};

#[derive(PartialEq, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    #[test]
    fn create_color() {
        let c = Color {
            r: -0.5,
            g: 0.4,
            b: 1.7,
        };
        assert_eq!(-0.5, c.r);
        assert_eq!(0.4, c.g);
        assert_eq!(1.7, c.b);
    }

    #[test]
    fn color_ops_addition() {
        let c1 = Color { r: 0.9, g: 0.6, b: 0.75, };
        let c2 = Color { r: 0.7, g: 0.1, b: 0.25, };

        let result = c1 + c2;
        let expected = Color { r: 1.6, g: 0.7, b: 1.0, };

        assert!((expected.r - result.r).abs() < std::f64::EPSILON);
        assert!((expected.g - result.g).abs() < std::f64::EPSILON);
        assert!((expected.b - result.b).abs() < std::f64::EPSILON);
    }

    #[test]
    fn color_ops_subtraction() {
        let c1 = Color { r: 0.9, g: 0.6, b: 0.75, };
        let c2 = Color { r: 0.7, g: 0.1, b: 0.25, };

        let result = c1 - c2;
        let expected = Color { r: 0.2, g: 0.5, b: 0.5, };

        assert!((expected.r - result.r).abs() < std::f64::EPSILON);
        assert!((expected.g - result.g).abs() < std::f64::EPSILON);
        assert!((expected.b - result.b).abs() < std::f64::EPSILON);
    }
}
