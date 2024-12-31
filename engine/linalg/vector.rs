#[derive(Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        return Vec3 { x: x, y: y, z: z };
    }
}

pub type Vec3f = Vec3<f32>;

#[cfg(test)]
pub(crate) mod test {
    fn test_vec3() {
        let v = Vec3<f32>::new(1.f, 2.f,3.f);
    }
}
