use glam::Vec3;
use glam::EulerRot;
use glam::Mat4;


#[derive(Default, Clone, Copy)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: EulerRot,
    pub scale: Vec3,

    pub matrix: Mat4,
    pub old_matrix: Mat4,
}

impl Transform {
    fn update_matrix(&mut self) {

        self.old_matrix =  self.matrix.clone();
        
        self.matrix = Mat4::from_scale_rotation_translation(
            -self.scale, 
            -glam::Quat::from_euler(self.rotation, 0.0, 0.0, 0.0), 
            -self.translation);
    }
}