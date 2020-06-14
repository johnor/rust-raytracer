use crate::{matrix, tuple};
use std::ops::Mul;

#[derive(Debug)]
pub struct Ray {
    pub origin: tuple::Tuple,
    pub direction: tuple::Tuple,
}

impl Ray {
    pub fn new(origin: tuple::Tuple, direction: tuple::Tuple) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> tuple::Tuple {
        self.origin + self.direction * t
    }
}

impl Mul<Ray> for matrix::Mat4x4 {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Ray {
        Ray::new(self * rhs.origin, self * rhs.direction)
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::transform;
    use crate::tuple::{point, vector};

    #[test]
    fn ray_create_and_query() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);

        let r = Ray::new(origin, direction);
        assert_eq!(origin, r.origin);
        assert_eq!(direction, r.direction);
    }

    #[test]
    fn ray_compute_point_from_distance() {
        let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));

        assert_eq!(point(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(point(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(point(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(point(4.5, 3.0, 4.0), r.position(2.5));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = transform::translate(3.0, 4.0, 5.0);
        let r2 = m * r;
        assert_eq!(point(4.0, 6.0, 8.0), r2.origin);
        assert_eq!(vector(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = transform::scale(2.0, 3.0, 4.0);
        let r2 = m * r;
        assert_eq!(point(2.0, 6.0, 12.0), r2.origin);
        assert_eq!(vector(0.0, 3.0, 0.0), r2.direction);
    }
}
