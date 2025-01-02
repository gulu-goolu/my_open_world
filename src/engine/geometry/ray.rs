use crate::linalg::vector::Vec3f;

#[derive(Debug, Default)]
pub struct Ray {
    pub origin: Vec3f,
    pub direct: Vec3f,
}
