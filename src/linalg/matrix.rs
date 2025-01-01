use crate::linalg::vector::Vec3f;
use std::ops::Mul;

#[derive(Debug, Default)]
pub struct Matrix3x3<T> {
    // row major
    pub m: [[T; 3]; 3],
}

impl Matrix3x3<f32> {
    pub fn zeros() -> Matrix3x3<f32> {
        return Matrix3x3 {
            m: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        };
    }

    pub fn identity() -> Matrix3x3<f32> {
        return Matrix3x3 {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        };
    }

    pub fn row(&self, row_idx: usize) -> Vec3f {
        return Vec3f::new(self.m[row_idx][0], self.m[row_idx][1], self.m[row_idx][2]);
    }
}

impl Mul for Matrix3x3<f32> {
    type Output = Matrix3x3<f32>;

    fn mul(self, other: Matrix3x3<f32>) -> Matrix3x3<f32> {
        let mut result: Matrix3x3<f32> = Default::default();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
pub mod tests {
    use super::Matrix3x3;

    #[test]
    fn test_mul() {
        let m1: Matrix3x3<f32> = Matrix3x3::identity();
        let m2: Matrix3x3<f32> = Matrix3x3::identity();
        let m3 = m1 * m2;

        let v0 = m3.row(0);
        assert_eq!(1.0, v0.x);
        assert_eq!(0.0, v0.y);
        assert_eq!(0.0, v0.z);
    }
}
