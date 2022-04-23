use std::f32::consts::PI;

use glam::{Mat4, mat4};

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

    pub fn get_vp_matrix(&self) -> Mat4 {
        self.get_view_matrix().mul_mat4(&self.get_projection_matrix())
    }

    fn get_view_matrix(&self) -> Mat4 {

        let new_matrix = Mat4::from_scale_rotation_translation(
            -self.transform.scale, 
            -glam::Quat::from_euler(self.transform.rotation, a, b, c), 
            -self.transform.translation);

        new_matrix
    }

    fn get_projection_matrix(&self) -> Mat4 {

    }
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
