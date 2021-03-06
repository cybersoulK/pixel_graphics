use glam::{Vec3, Mat4};

use super::world_orientation::get_quat;


#[derive(Clone, Copy)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,

    pub matrix: Mat4,
    pub old_matrix: Mat4,
}

impl Transform {
    pub fn update_matrix(&mut self) {

        self.old_matrix =  self.matrix.clone();
        
        self.matrix = Mat4::from_scale_rotation_translation(
            self.scale, 
            get_quat(self.rotation),
            self.translation);
    }
}


impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            rotation: Default::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            matrix: Default::default(),
            old_matrix: Default::default(),
        }
    }
}