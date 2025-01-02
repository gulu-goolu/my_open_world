use super::matrix::Matrix4x4;
use std::ops::Mul;

pub struct Transform {
    pub local_to_world: Matrix4x4<f32>,
    pub world_to_local: Matrix4x4<f32>,
}

impl Transform {
    pub fn identity() -> Transform {
        return Transform {
            local_to_world: Matrix4x4::identity(),
            world_to_local: Matrix4x4::identity(),
        };
    }
}

impl Mul for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Transform {
        return Transform {
            local_to_world: self.local_to_world * rhs.local_to_world,
            world_to_local: self.world_to_local * rhs.world_to_local,
        };
    }
}
