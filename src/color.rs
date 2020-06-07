use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
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

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use crate::color::Color;

    pub fn assert_color_eq(c1: Color, c2: Color) {
        assert!((c1.r - c2.r).abs() < std::f64::EPSILON);
        assert!((c1.g - c2.g).abs() < std::f64::EPSILON);
        assert!((c1.b - c2.b).abs() < std::f64::EPSILON);
    }
}

#[cfg(test)]
mod tests {
    use crate::color::test_utils::assert_color_eq;
    use crate::color::Color;

    #[test]
    fn create_color() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(-0.5, c.r);
        assert_eq!(0.4, c.g);
        assert_eq!(1.7, c.b);
    }

    #[test]
    fn color_ops_addition() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_color_eq(Color::new(1.6, 0.7, 1.0), c1 + c2);
    }

    #[test]
    fn color_ops_subtraction() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_color_eq(Color::new(0.2, 0.5, 0.5), c1 - c2);
    }

    #[test]
    fn color_ops_multiply_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_color_eq(Color::new(0.9, 0.2, 0.04), c1 * c2);
    }

    #[test]
    fn color_ops_multiply_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_color_eq(Color::new(0.4, 0.6, 0.8), c * 2.0);
    }
}
