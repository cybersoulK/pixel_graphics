use std::f32::consts::PI;

use glam::{Mat4};

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

        self.transform.matrix
    }

    fn get_projection_matrix(&self) -> Mat4 {

        let matrix_array = [
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        ];

        let matrix = Mat4::from_cols_array(&matrix_array);
        //Bowen Rust Learning Task: 
        //search Mat4:: static functions, and view alternatives to the ::from_cols_array 
        // 😚💖💖

        matrix
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
