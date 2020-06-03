use std::ops::{Add, Sub};

#[derive(PartialEq, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        (self.w - 1.0).abs() < std::f64::EPSILON
    }

    pub fn is_vector(&self) -> bool {
        self.w.abs() < std::f64::EPSILON
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

#[cfg(test)]
mod tests {
    use crate::tuple::point;
    use crate::tuple::vector;
    use crate::tuple::Tuple;

    fn assert_tuple_eq(t1: Tuple, t2: Tuple) {
        assert!((t1.x - t2.x).abs() < std::f64::EPSILON);
        assert!((t1.y - t2.y).abs() < std::f64::EPSILON);
        assert!((t1.z - t2.z).abs() < std::f64::EPSILON);
        assert!((t1.w - t2.w).abs() < std::f64::EPSILON);
    }

    #[test]
    fn tuple_with_w1_is_a_point() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(4.3, t.x);
        assert_eq!(-4.2, t.y);
        assert_eq!(3.1, t.z);
        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    fn tuple_with_w0_is_a_vector() {
        let t = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(4.3, t.x);
        assert_eq!(-4.2, t.y);
        assert_eq!(3.1, t.z);
        assert!(!t.is_point());
        assert!(t.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w1() {
        let p = point(4.0, -4.0, 3.0);
        let t = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };
        assert_eq!(t, p);
    }

    #[test]
    fn vector_creates_tuples_with_w0() {
        let p = vector(4.0, -4.0, 3.0);
        let t = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0.0,
        };
        assert_eq!(t, p);
    }

    #[test]
    fn add_two_tuples() {
        let t1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let t2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        assert_tuple_eq(
            Tuple {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 1.0,
            },
            t1 + t2,
        );
    }

    #[test]
    fn subtract_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_tuple_eq(vector(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_tuple_eq(point(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_tuple_eq(vector(-2.0, -4.0, -6.0), v1 - v2);
    }
}
