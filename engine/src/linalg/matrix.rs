use crate::linalg::vector::Vec3f;
use std::ops::Mul;

use super::vector::Vec4f;

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

    // matrix row
    pub fn row(self, row_idx: usize) -> Vec3f {
        return Vec3f::new(self.m[row_idx][0], self.m[row_idx][1], self.m[row_idx][2]);
    }

    // matrix column
    pub fn col(self, col: usize) -> Vec3f {
        return Vec3f::new(self.m[0][col], self.m[1][col], self.m[2][col]);
    }
}

/// row major matrix
#[derive(Debug, Default)]
pub struct Matrix4x4<T> {
    pub m: [[T; 4]; 4],
}

impl Matrix4x4<f32> {
    pub fn zeros() -> Matrix4x4<f32> {
        return Matrix4x4 {
            m: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };
    }

    pub fn from_rows(rows: [Vec4f; 4]) -> Matrix4x4<f32> {
        return Matrix4x4 {
            m: [
                [rows[0].x, rows[0].y, rows[0].z, rows[0].w],
                [rows[1].x, rows[1].y, rows[1].z, rows[1].w],
                [rows[2].x, rows[2].y, rows[2].z, rows[2].w],
                [rows[3].x, rows[3].y, rows[3].z, rows[3].w],
            ],
        };
    }

    pub fn identity() -> Matrix4x4<f32> {
        return Matrix4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
    }

    pub fn transpose(self) -> Matrix4x4<f32> {
        return Matrix4x4 {
            m: [
                [self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]],
                [self.m[0][1], self.m[1][1], self.m[2][2], self.m[3][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]],
                [self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]],
            ],
        };
    }

    pub fn row(self, row: usize) -> Vec4f {
        return Vec4f {
            x: self.m[row][0],
            y: self.m[row][1],
            z: self.m[row][2],
            w: self.m[row][3],
        };
    }

    pub fn col(self, col: usize) -> Vec4f {
        return Vec4f {
            x: self.m[0][col],
            y: self.m[1][col],
            z: self.m[2][col],
            w: self.m[3][col],
        };
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

impl Mul for Matrix4x4<f32> {
    type Output = Matrix4x4<f32>;

    fn mul(self, rhs: Matrix4x4<f32>) -> Matrix4x4<f32> {
        let mut result: Matrix4x4<f32> = Default::default();

        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result.m[j][j] += self.m[i][k] * rhs.m[k][j];
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::linalg::vector::Vec4f;

    use super::{Matrix3x3, Matrix4x4};

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

    #[test]
    fn test_transpose() {
        let v = Vec4f::new(1.0, 2.0, 3.0, 4.0);
        let m = Matrix4x4::from_rows([v, v, v, v]);
        let m2 = m.transpose();

        let col0 = m2.col(0);
        assert_eq!(1.0, col0.x);
    }
}
