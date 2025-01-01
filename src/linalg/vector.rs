use std::ops::{Add, Mul, Sub};

#[derive(Debug, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        return Vec3 { x: x, y: y, z: z };
    }
}

impl Vec3f {
    pub fn length(&self) -> f32 {
        let s = self.x * self.x + self.y * self.y + self.z * self.z;
        return s.sqrt();
    }
}

// dot product
pub fn dot3<T>(v1: Vec3<T>, v2: Vec3<T>) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
{
    return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
}

pub fn dot4<T>(v1: Vec4<T>, v2: Vec4<T>) -> T
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
{
    return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w;
}

// cross product
pub fn cross3<T>(v1: Vec3<T>, v2: Vec3<T>) -> Vec3<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    return Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    };
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
