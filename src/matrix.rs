macro_rules! define_matrix_struct {
    ($name:ident, $size:expr) =>
    {
        #[derive(Clone, Copy, Debug)]
        pub struct $name {
            pub data: [[f64; $size]; $size]
        }

        impl $name {
           fn new(data: [[f64; $size]; $size]) -> Self {
              Self { data }
           }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                for i in 0..$size {
                    for j in 0..$size {
                        if (self.data[i][j] - other.data[i][j]).abs() > std::f64::EPSILON {
                            return false
                        }
                    }
                }
                return true
            }
        }

        impl Eq for $name {}
    };
}

define_matrix_struct!(Mat2x2, 2);
define_matrix_struct!(Mat3x3, 3);
define_matrix_struct!(Mat4x4, 4);

#[cfg(test)]
mod tests {
    use crate::matrix::Mat2x2;
    use crate::matrix::Mat3x3;
    use crate::matrix::Mat4x4;

    #[test]
    fn create_2x2_matrix() {
        let m = Mat2x2::new([
            [-3.0, 5.0],
            [1.0, -2.0]
        ]);
        assert_eq!(-3.0, m.data[0][0]);
        assert_eq!(5.0, m.data[0][1]);
        assert_eq!(1.0, m.data[1][0]);
        assert_eq!(-2.0, m.data[1][1]);
    }

    #[test]
    fn create_3x3_matrix() {
        let m = Mat3x3::new([
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0],
        ]);
        assert_eq!(-3.0, m.data[0][0]);
        assert_eq!(-2.0, m.data[1][1]);
        assert_eq!(1.0, m.data[2][2]);
    }

    #[test]
    fn create_4x4_matrix() {
        let m = Mat4x4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]
        ]);
        assert_eq!(1.0, m.data[0][0]);
        assert_eq!(4.0, m.data[0][3]);
        assert_eq!(5.5, m.data[1][0]);
        assert_eq!(7.5, m.data[1][2]);
        assert_eq!(11.0, m.data[2][2]);
        assert_eq!(13.5, m.data[3][0]);
        assert_eq!(15.5, m.data[3][2]);
    }

    #[test]
    fn matrix_equality() {
        let data = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]
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
            [5.0, 4.0, 3.0, 2.0]
        ]);
        let b = Mat4x4::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0]
        ]);
        assert_ne!(a, b);
    }
}