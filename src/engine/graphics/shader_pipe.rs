use std::rc::Rc;

use glam::{Vec3, Vec2, Mat4};

use super::super::{Light, Material};

pub struct ShaderPipe<'a> {
    pub vp_matrix: Mat4,
    pub model_matrix: Mat4,
    
    pub vertices: [Vec3; 3],
    pub uv_textures: [Vec2; 3],
    pub norms: [Vec3; 3],

    pub material: Rc<Material>,
    pub lights: &'a Vec<Rc<Light>>,
}