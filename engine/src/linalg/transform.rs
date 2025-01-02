use super::matrix::Matrix4x4;

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
