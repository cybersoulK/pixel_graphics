use std::f32::consts::PI;

use super::{Object, Transform};


pub struct Camera {
    transform: Transform,

    near: f32,
    far: f32,
    fov: f32,
}

impl Camera {
    pub fn new(transform: Transform, near: f32, far: f32, fov: f32) -> Self {
        Self { transform, near, far, fov }
    }
    //fn get_projection_matrix...
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Transform::default(),

            near: 0.1,
            far: 10000.0,
            fov: PI / 2.0,
        }
    }
}

impl Object for Camera {
}
