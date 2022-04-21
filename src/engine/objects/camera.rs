use super::{Object, Transform};

pub struct Camera {
    transform: Transform,

    near: f32,
    far: f32,
    FOV: f32,
}

impl Camera {
    pub fn new(transform: Transform, near: f32, far: f32, FOV: f32) -> Self {
        Self { transform, near, far, FOV }
    }
    //fn get_projection_matrix...
}

impl Object for Camera {
}
