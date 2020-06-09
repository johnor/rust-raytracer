use crate::tuple::Tuple;
use std::ops::{Index, IndexMut, Mul};
use std::vec::Vec;

#[derive(Clone, Debug)]
pub struct SquareMatrix {
    data: Vec<Vec<f64>>,
}

impl SquareMatrix {
    fn new2x2(m: [[f64; 2]; 2]) -> Self {
        let mut data = Vec::new();
        data.push(m[0].to_vec());
        data.push(m[1].to_vec());
        Self { data }
    }

    fn new3x3(m: [[f64; 3]; 3]) -> Self {
        let mut data = Vec::new();
        data.push(m[0].to_vec());
        data.push(m[1].to_vec());
        data.push(m[2].to_vec());
        Self { data }
    }

    fn new4x4(m: [[f64; 4]; 4]) -> Self {
        let mut data = Vec::new();
        data.push(m[0].to_vec());
        data.push(m[1].to_vec());
        data.push(m[2].to_vec());
        data.push(m[3].to_vec());
        Self { data }
    }

    fn from_size(size: usize) -> Self {
        Self {
            data: vec![vec![0.0; size]; size],
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    pub fn transpose(&self) -> Self {
        let mut m = Self::from_size(self.size());
        for r in 0..self.size() {
            for c in 0..self.size() {
                m[c][r] = self[r][c]
            }
        }
        m
    }
}

impl PartialEq for SquareMatrix {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..self.size() {
            for c in 0..self.size() {
                if (self[r][c] - other[r][c]).abs() > std::f64::EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for SquareMatrix {}

impl Index<usize> for SquareMatrix {
    type Output = Vec<f64>;

    fn index(&self, r: usize) -> &Vec<f64> {
        &self.data[r]
    }
}

impl IndexMut<usize> for SquareMatrix {
    fn index_mut(&mut self, r: usize) -> &mut Vec<f64> {
        &mut self.data[r]
    }
}

impl Mul for SquareMatrix {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut m = SquareMatrix::from_size(self.size());
        for r in 0..self.size() {
            for c in 0..self.size() {
                let mut v = 0.0;
                for i in 0..self.size() {
                    v += self[r][i] * other[i][c];
                }
                m[r][c] = v;
            }
        }
        m
    }
}

impl Mul<Tuple> for SquareMatrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Tuple {
        let mut res = vec![0.0; 4];
        for r in 0..res.len() {
            res[r] = Tuple::from_vec(self[r].clone()).dot(&rhs);
        }
        Tuple::from_vec(res)
    }
}

pub fn identity_matrix(size: usize) -> SquareMatrix {
    let mut m = SquareMatrix::from_size(size);
    for i in 0..m.size() {
        m[i][i] = 1.0;
    }
    m
}

#[cfg(test)]
mod tests {
    use crate::matrix::{identity_matrix, SquareMatrix};
    use crate::tuple::{point, Tuple};

    #[test]
    fn create_2x2_matrix() {
        let m = SquareMatrix::new2x2([[-3.0, 5.0], [1.0, -2.0]]);
        assert_eq!(-3.0, m[0][0]);
        assert_eq!(5.0, m[0][1]);
        assert_eq!(1.0, m[1][0]);
        assert_eq!(-2.0, m[1][1]);
    }

    #[test]
    fn create_3x3_matrix() {
        let m = SquareMatrix::new3x3([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert_eq!(-3.0, m[0][0]);
        assert_eq!(-2.0, m[1][1]);
        assert_eq!(1.0, m[2][2]);
    }

    #[test]
    fn create_4x4_matrix() {
        let m = SquareMatrix::new4x4([
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
        let a = SquareMatrix::new4x4(data);
        let b = SquareMatrix::new4x4(data);
        assert_eq!(a, b);
    }

    #[test]
    fn matrix_inequality() {
        let a = SquareMatrix::new4x4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = SquareMatrix::new4x4([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(a, b);
    }

    #[test]
    fn multiply_two_matrices() {
        let a = SquareMatrix::new4x4([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = SquareMatrix::new4x4([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let expected = SquareMatrix::new4x4([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(expected, a * b);
    }

    #[test]
    fn matrix_multiplied_by_tuple() {
        let a = SquareMatrix::new4x4([
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
        let a = SquareMatrix::new2x2([[0.0, 1.0], [1.0, 2.0]]);
        assert_eq!(a.clone(), a * identity_matrix(2));
    }

    #[test]
    fn matrix_3x3_multiplied_by_identity_matrix() {
        let a = SquareMatrix::new3x3([[0.0, 1.0, 2.0], [1.0, 2.0, 4.0], [2.0, 4.0, 8.0]]);
        assert_eq!(a.clone(), a * identity_matrix(3));
    }

    #[test]
    fn matrix_4x4_multiplied_by_identity_matrix() {
        let a = SquareMatrix::new4x4([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        assert_eq!(a.clone(), a * identity_matrix(4));
    }

    #[test]
    fn identity_matrix_multiplied_by_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a, identity_matrix(4) * a);
    }

    #[test]
    fn transpose_a_matrix() {
        let a = SquareMatrix::new4x4([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let expected = SquareMatrix::new4x4([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(expected, a.transpose());
    }

    #[test]
    fn transpose_identify_matrix() {
        let a = identity_matrix(4);
        assert_eq!(a, a.transpose());
    }
}
