use crate::tuple::Tuple;
use std::ops::{Index, IndexMut, Mul};

macro_rules! define_square_matrix_struct {
    ($name:ident, $order:expr) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name {
            pub data: [[f64; $order]; $order],
        }

        impl $name {
            pub fn new(data: [[f64; $order]; $order]) -> Self {
                Self { data }
            }

            pub fn zero() -> Self {
                let data: [[f64; $order]; $order] = [[0.0; $order]; $order];
                Self::new(data)
            }

            pub fn identity() -> Self {
                let mut result = Self::zero();
                for i in 0..$order {
                    result[i][i] = 1.0;
                }
                result
            }

            pub fn transpose(&self) -> Self {
                let mut data = [[0.0; $order]; $order];
                for r in 0..$order {
                    for c in 0..$order {
                        data[c][r] = self.data[r][c]
                    }
                }
                Self::new(data)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                for r in 0..$order {
                    for c in 0..$order {
                        if (self.data[r][c] - other.data[r][c]).abs() > std::f64::EPSILON {
                            return false;
                        }
                    }
                }
                return true;
            }
        }

        impl Eq for $name {}

        impl Index<usize> for $name {
            type Output = [f64; $order];

            fn index(&self, r: usize) -> &[f64; $order] {
                &self.data[r]
            }
        }

        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, r: usize) -> &mut [f64; $order] {
                &mut self.data[r]
            }
        }

        impl Mul for $name {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                let mut data = [[0.0; $order]; $order];
                for r in 0..$order {
                    for c in 0..$order {
                        let mut v = 0.0;
                        for i in 0..$order {
                            v += self.data[r][i] * other.data[i][c];
                        }
                        data[r][c] = v;
                    }
                }
                Self::new(data)
            }
        }
    };
}

define_square_matrix_struct!(Mat2x2, 2);
define_square_matrix_struct!(Mat3x3, 3);
define_square_matrix_struct!(Mat4x4, 4);

impl Mul<Tuple> for Mat4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        let mut res = [0.0; 4];
        for r in 0..4 {
            res[r] = Tuple::from_array(self[r]).dot(&rhs);
        }
        Tuple::from_array(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Mat2x2;
    use crate::matrix::Mat3x3;
    use crate::matrix::Mat4x4;
    use crate::tuple::{point, Tuple};

    #[test]
    fn create_2x2_matrix() {
        let m = Mat2x2::new([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(-3.0, m[0][0]);
        assert_eq!(5.0, m[0][1]);
        assert_eq!(1.0, m[1][0]);
        assert_eq!(-2.0, m[1][1]);
    }

    #[test]
    fn create_3x3_matrix() {
        let m = Mat3x3::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert_eq!(-3.0, m[0][0]);
        assert_eq!(-2.0, m[1][1]);
        assert_eq!(1.0, m[2][2]);
    }

    #[test]
    fn create_4x4_matrix() {
        let m = Mat4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(1.0, m[0][0]);
        assert_eq!(4.0, m[0][3]);
        assert_eq!(5.5, m[1][0]);
        assert_eq!(7.5, m[1][2]);
        assert_eq!(11.0, m[2][2]);
        assert_eq!(13.5, m[3][0]);
        assert_eq!(15.5, m[3][2]);
    }

    #[test]
    fn matrix_equality() {
        let data = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let a = Mat4x4::new(data);
        let b = Mat4x4::new(data);
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_inequality() {
        let a = Mat4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Mat4x4::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(a, b);
    }

    #[test]
    fn multiply_two_matrices() {
        let a = Mat4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Mat4x4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let expected = Mat4x4::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(expected, a * b);
    }

    #[test]
    fn matrix_multiplied_by_tuple() {
        let a = Mat4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = point(1.0, 2.0, 3.0);
        assert_eq!(point(18.0, 24.0, 33.0), a * b);
    }

    #[test]
    fn matrix_2x2_multiplied_by_identity_matrix() {
        let a = Mat2x2::new([[0.0, 1.0], [1.0, 2.0]]);
        assert_eq!(a, a * Mat2x2::identity());
    }

    #[test]
    fn matrix_3x3_multiplied_by_identity_matrix() {
        let a = Mat3x3::new([[0.0, 1.0, 2.0], [1.0, 2.0, 4.0], [2.0, 4.0, 8.0]]);
        assert_eq!(a, a * Mat3x3::identity());
    }

    #[test]
    fn matrix_4x4_multiplied_by_identity_matrix() {
        let a = Mat4x4::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert_eq!(a, a * Mat4x4::identity());
    }

    #[test]
    fn identity_matrix_multipled_by_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a, Mat4x4::identity() * a);
    }

    #[test]
    fn transpose_a_matrix() {
        let a = Mat4x4::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let expected = Mat4x4::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(expected, a.transpose());
    }
}
