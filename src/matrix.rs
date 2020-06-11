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

            pub fn order() -> usize {
                $order
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
                        if (self.data[r][c] - other.data[r][c]).abs() > 0.0000000001 {
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

macro_rules! impl_sub_matrix {
    ($parent:ident, $child:ident) => {
        impl $parent {
            fn submatrix(&self, row: usize, col: usize) -> $child {
                let mut res = $child::zero();
                for i in 0..$parent::order() {
                    if i != row {
                        let k = if i < row { 0 } else { 1 };
                        for j in 0..$parent::order() {
                            if j != col {
                                let l = if j < col { 0 } else { 1 };
                                res[i - k][j - l] = self[i][j];
                            }
                        }
                    }
                }
                res
            }
        }
    };
}

macro_rules! impl_determinant {
    ($parent:ident) => {
        impl $parent {
            fn minor(&self, row: usize, col: usize) -> f64 {
                self.submatrix(row, col).determinant()
            }

            fn cofactor(&self, row: usize, col: usize) -> f64 {
                let minor = self.minor(row, col);
                if (row + col) % 2 == 0 {
                    minor
                } else {
                    -minor
                }
            }

            fn determinant(&self) -> f64 {
                let mut res = 0.0;
                for i in 0..Self::order() {
                    res += (self[0][i] * self.cofactor(0, i));
                }
                res
            }
        }
    };
}

define_square_matrix_struct!(Mat2x2, 2);
define_square_matrix_struct!(Mat3x3, 3);
define_square_matrix_struct!(Mat4x4, 4);
impl_sub_matrix!(Mat4x4, Mat3x3);
impl_sub_matrix!(Mat3x3, Mat2x2);
impl_determinant!(Mat3x3);
impl_determinant!(Mat4x4);

impl Mat2x2 {
    fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Mat4x4 {
    fn invertible(&self) -> bool {
        self.determinant().abs() > std::f64::EPSILON
    }

    fn inverse(&self) -> Result<Self, &str> {
        if self.invertible() {
            let mut res = Mat4x4::zero();
            let det = self.determinant();
            for r in 0..Self::order() {
                for c in 0..Self::order() {
                    let cof = self.cofactor(r, c);
                    res[c][r] = cof / det;
                }
            }
            Ok(res)
        } else {
            Err("Matrix is not invertible")
        }
    }
}

impl Mul<Tuple> for Mat4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        let mut res = [0.0; 4];
        for r in 0..4 {
            res[r] = Tuple::from_array(self[r]).dot(rhs);
        }
        Tuple::from_array(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{Mat2x2, Mat3x3, Mat4x4};
    use crate::tuple::{point, Tuple};

    fn assert_mat4x4_near(a: Mat4x4, b: Mat4x4) {
        for r in 0..Mat4x4::order() {
            for c in 0..Mat4x4::order() {
                assert!((b[r][c] - a[r][c]).abs() < 0.00001);
            }
        }
    }

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
    fn identity_matrix_multiplied_by_tuple() {
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

    #[test]
    fn transpose_identify_matrix() {
        let a = Mat4x4::identity();
        assert_eq!(a, a.transpose());
    }

    #[test]
    fn determinant_of_2x2_matrix() {
        let a = Mat2x2::new([[1., 5.], [-3., 2.]]);
        assert_eq!(17.0, a.determinant());
    }

    #[test]
    fn submatrix_of_3x3() {
        let a = Mat3x3::new([[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]]);
        let expected = Mat2x2::new([[-3., 2.], [0., 6.]]);
        assert_eq!(expected, a.submatrix(0, 2));
    }

    #[test]
    fn submatrix_of_4x4() {
        let a = Mat4x4::new([
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.],
        ]);
        let expected = Mat3x3::new([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);
        assert_eq!(expected, a.submatrix(2, 1));
    }

    #[test]
    fn minor_of_3x3() {
        let a = Mat3x3::new([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);
        let b = a.submatrix(1, 0);
        assert_eq!(25., b.determinant());
        assert_eq!(25., a.minor(1, 0));
    }

    #[test]
    fn cofactor_of_3x3() {
        let a = Mat3x3::new([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);
        assert_eq!(-12., a.minor(0, 0));
        assert_eq!(-12., a.cofactor(0, 0));
        assert_eq!(25., a.minor(1, 0));
        assert_eq!(-25., a.cofactor(1, 0));
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let a = Mat3x3::new([[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]]);
        assert_eq!(56., a.cofactor(0, 0));
        assert_eq!(12., a.cofactor(0, 1));
        assert_eq!(-46., a.cofactor(0, 2));
        assert_eq!(-196., a.determinant());
    }

    #[test]
    fn determinant_of_4x4_matrix() {
        let a = Mat4x4::new([
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.],
        ]);
        assert_eq!(690., a.cofactor(0, 0));
        assert_eq!(447., a.cofactor(0, 1));
        assert_eq!(210., a.cofactor(0, 2));
        assert_eq!(51., a.cofactor(0, 3));
        assert_eq!(-4071., a.determinant());
    }

    #[test]
    fn invertible_matrix_is_invertible() {
        let a = Mat4x4::new([
            [6., 4., 4., 4.],
            [5., 5., 7., 6.],
            [4., -9., 3., -7.],
            [9., 1., 7., -6.],
        ]);
        assert_eq!(-2120., a.determinant());
        assert_eq!(true, a.invertible());
    }

    #[test]
    fn noninvertible_matrix_is_not_invertible() {
        let a = Mat4x4::new([
            [-4., 2., -2., -3.],
            [9., 6., 2., 6.],
            [0., -5., 1., -5.],
            [0., 0., 0., 0.],
        ]);
        assert_eq!(0., a.determinant());
        assert_eq!(false, a.invertible());
    }

    #[test]
    fn inverse_of_matrix() {
        let a = Mat4x4::new([
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.],
        ]);
        let expected = Mat4x4::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        let b = a.inverse().unwrap();
        assert_eq!(532., a.determinant());
        assert_eq!(-160., a.cofactor(2, 3));
        assert_eq!(-160. / 532., b[3][2]);
        assert_eq!(105., a.cofactor(3, 2));
        assert_eq!(105. / 532., b[2][3]);
        assert_mat4x4_near(expected, b);
    }

    #[test]
    fn inverse_of_another_matrix() {
        let a = Mat4x4::new([
            [8., -5., 9., 2.],
            [7., 5., 6., 1.],
            [-6., 0., 9., 6.],
            [-3., 0., -9., -4.],
        ]);
        let expected = Mat4x4::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_mat4x4_near(expected, a.inverse().unwrap());
    }

    #[test]
    fn inverse_of_a_third_matrix() {
        let a = Mat4x4::new([
            [9., 3., 0., 9.],
            [-5., -2., -6., -3.],
            [-4., 9., 6., 4.],
            [-7., 6., 6., 2.],
        ]);
        let expected = Mat4x4::new([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert_mat4x4_near(expected, a.inverse().unwrap());
    }

    #[test]
    fn inverse_of_a_noninvertible_matrix() {
        let a = Mat4x4::new([
            [-4., 2., -2., -3.],
            [9., 6., 2., 6.],
            [0., -5., 1., -5.],
            [0., 0., 0., 0.],
        ]);
        match a.inverse() {
            Err(s) => assert_eq!("Matrix is not invertible", s),
            _ => assert!(false),
        }
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = Mat4x4::new([
            [3., -9., 7., 3.],
            [3., -8., 2., -9.],
            [-4., 4., 4., 1.],
            [-6., 5., -1., 1.],
        ]);
        let b = Mat4x4::new([
            [8., 2., 2., 2.],
            [3., -1., 7., 0.],
            [7., 0., 5., 4.],
            [6., -2., 0., 5.],
        ]);
        let c = a * b;
        let d = c * b.inverse().unwrap();
        assert_eq!(a, d);
    }
}
