use std::{f32::consts::PI};

use glam::{Vec2, Mat4};

use super::{Object, Transform, ComponentVec};


pub struct Camera {
    transform: Transform,
    components: ComponentVec,

    near: f32,
    far: f32,

    fov: f32,
}

impl Camera {
    pub fn new(transform: Transform, near: f32, fov: f32, ) -> Self {
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

    
    pub fn get_view_matrix(&self) -> Mat4 {
        self.transform.matrix
    }

    //aspect ratio is x / y
    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> Mat4 {

        Mat4::perspective_lh(self.fov, aspect_ratio, self.near, self.far)
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


impl Object for Camera {
    fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn components(&mut self) -> &mut ComponentVec {
        &mut self.components
    }

    fn get_update_bundle(&mut self) -> (&mut Transform, &ComponentVec) {
        (&mut self.transform, &self.components)
    }
}