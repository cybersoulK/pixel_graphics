use std::rc::Rc;

use glam::Vec4;

use super::{Shader, VertexPipe, FragmentPipe};


pub struct Shader1 {

}

impl Shader1 {
    pub fn new() -> Rc<Shader1> {
        Rc::new(Self {

        })
    }
}

impl Shader for Shader1 {

    fn vertex_shader(&self, inputs: VertexPipe) -> VertexPipe {
        
        let mut outputs = inputs;
        
        outputs.color = match outputs.id {
            0 => Some(Vec4::new(255.0, 0.0, 0.0, 255.0)),
            1 => Some(Vec4::new(0.0, 0.0, 255.0, 255.0)),
            _ => Some(Vec4::new(0.0, 255.0, 0.0, 255.0)),
        };

        outputs.vertex.y = -outputs.vertex.y + 600.0;

        outputs
    }

    fn fragment_shader(&self, inputs: FragmentPipe) -> Vec4 {
     
        inputs.color.unwrap()
    }
}