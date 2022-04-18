use glam::Vec3;
use glam::EulerRot;
use glam::Mat4;

use super::assets::Model;


#[derive(Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: EulerRot,
    pub scale: Vec3,

    pub matrix: Mat4,
}

pub struct Camera {
    transform: Transform,

    near: f32,
    far: f32,
    angle: f32,
}

impl Camera {
    pub fn new(transform: Transform, near: f32, far: f32, angle: f32) -> Self {
        Self { transform, near, far, angle }
    }
    //fn get_projection_matrix...
}

pub struct DrawableObject {
    transform: Transform,

    model: Model,
}

impl DrawableObject {
    pub fn new(transform: Transform, model: Model) -> Self {
        Self { transform, model }
    }
}



