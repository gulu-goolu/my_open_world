use std::ops::{Add, Sub};

#[derive(Debug, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        return Vec3 { x: x, y: y, z: z };
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Self) -> Self {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Self) -> Self {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

pub type Vec3f = Vec3<f32>;

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_new() {
        let v1: Vec3<f32> = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, v1.x);
        assert_eq!(2.0, v1.y);
        assert_eq!(3.0, v1.z);
    }
}
