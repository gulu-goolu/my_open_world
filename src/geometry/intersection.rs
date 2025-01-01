use super::ray::Ray;
use crate::linalg::vector::Vec3f;
use std::option::Option;

pub struct Interaction {
    pub p: Vec3f,
}

pub trait Geometry {
    fn hit(&self, ray: &Ray) -> Option<Interaction>;
}
