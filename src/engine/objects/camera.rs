use std::{f32::consts::PI};

use glam::{Vec2, Mat4};

use super::{Transform, ComponentVec};


pub struct Camera {
    pub transform: Transform,
    pub components: ComponentVec,

    near: f32,
    far: f32,

    fov: f32,
}

impl Camera {
    pub fn new(transform: Transform, near: f32, fov: f32) -> Self {
        Self {
            transform,
            components: ComponentVec::new(),
            
            near,
            far: 10000.0,

            fov,
        }
    }

    pub fn get_near(&self) -> f32 {
        self.near
    }

    pub fn get_vp_matrix(&self, buffer_size: Vec2) -> Mat4 {

        let aspect_ratio = buffer_size.y / buffer_size.x;

        Mat4::default()
            .mul_mat4(&self.get_view_matrix().inverse())
            .mul_mat4(&self.get_projection_matrix(aspect_ratio))
    }

    fn get_view_matrix(&self) -> Mat4 {

        self.transform.matrix
    }

    fn get_projection_matrix(&self, aspect_ratio: f32) -> Mat4 {

        Mat4::from_cols_array_2d(&[
            [1.0 / (aspect_ratio * (self.fov/2.0).tan()), 0.0, 0.0, 0.0],
            [0.0, 1.0 / ((self.fov/2.0).tan()), 0.0, 0.0],
            [0.0, 0.0, - (self.near+self.far) / (self.near-self.far), - (2.0*self.near*self.far) / (self.near-self.far)],
            [0.0, 0.0, -1.0, 0.0]
            ])
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            components: ComponentVec::new(),

            near: 0.1,
            far: 10000.0,

            fov: PI / 2.0,
        }
    }
}

