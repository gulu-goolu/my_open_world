use crate::linalg::transform;

pub struct SceneNode {
    pub transform: transform::Transform,

    pub children: Vec<SceneNode>,
}
