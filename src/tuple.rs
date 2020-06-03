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

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 1.0,
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::point;
    use crate::tuple::vector;
    use crate::tuple::Tuple;

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
}
