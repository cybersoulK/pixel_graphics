use std::rc::Rc;

use glam::{Vec4};

use pixel_graphics::{Shader, CorePipe, ParamsPipe, DynamicPipe};


pub struct CustomShader {

}

impl CustomShader {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {

        })
    }
}


impl Shader for CustomShader {

    fn vertex_shader(&self, mut core: CorePipe, params: &ParamsPipe, dynamic: &mut DynamicPipe, face_id: usize, vertex_id: usize) -> CorePipe {
        
        core.color = match face_id {
            0 | 1 => Vec4::new(0.71, 0.0, 0.0, 1.0),
            2 | 3 => Vec4::new(0.0, 0.0, 0.78, 1.0),
            4 | 5 => Vec4::new(0.71, 0.71, 0.71, 1.0),
            6 | 7 => Vec4::new(0.0, 0.78, 0.78, 1.0),
            8 | 9 => Vec4::new(0.78, 0.0, 0.78, 1.0),
            _ => Vec4::new(0.71, 0.71, 0.0, 1.0),
        };

        core
    }
}


pub fn build() -> Rc<CustomShader> {
    CustomShader::new()
}