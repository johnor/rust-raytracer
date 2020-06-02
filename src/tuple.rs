#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    #[test]
    fn tuple_with_w1_is_a_point() {
        let t = Tuple {
            x: 42.0,
            y: -1.2,
            z: 100.12,
            w: 1.0,
        };
        assert_eq!(42.0, t.x);
        assert_eq!(-1.2, t.y);
        assert_eq!(100.12, t.z);
        assert!(t.is_point());
        assert!(!t.is_vector());
    }
    #[test]
    fn tuple_with_w0_is_a_vector() {
        let t = Tuple {
            x: 42.0,
            y: -1.2,
            z: 100.12,
            w: 0.0,
        };
        assert_eq!(42.0, t.x);
        assert_eq!(-1.2, t.y);
        assert_eq!(100.12, t.z);
        assert!(!t.is_point());
        assert!(t.is_vector());
    }
}
