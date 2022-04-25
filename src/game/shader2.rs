use std::rc::Rc;

use glam::{Vec3, Vec4};

use super::{Shader, CorePipe, VertexPipe, FragmentPipe};


pub struct Shader2 {

}

impl Shader2 {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {

        })
    }
}


impl Shader for Shader2 {

    fn vertex_shader(&self, mut core: CorePipe, params: &VertexPipe, vertex_id: usize, face_id: usize) -> CorePipe {
        
        core.color = match face_id {
            0 | 1 => Some(Vec4::new(255.0, 0.0, 0.0, 255.0)),
            2 | 3 => Some(Vec4::new(0.0, 0.0, 255.0, 255.0)),
            4 | 5 => Some(Vec4::new(255.0, 0.0, 0.0, 255.0)),
            6 | 7 => Some(Vec4::new(0.0, 100.0, 100.0, 255.0)),
            8 | 9 => Some(Vec4::new(200.0, 0.0, 255.0, 255.0)),
            _ => Some(Vec4::new(100.0, 100.0, 0.0, 255.0)),
        };

        core.vertex *= 1000.0;
        core.vertex.y = -core.vertex.y + 400.0;

        core
    }

    fn fragment_shader(&self, core: CorePipe, _params: &FragmentPipe, vertex_2d: Vec3) -> Vec4 {
    
        core.color.unwrap()
    }
}