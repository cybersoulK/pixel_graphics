use std::rc::Rc;

use glam::Vec4;

use super::{Shader, CorePipe, VertexPipe, FragmentPipe};


pub struct Shader1 {

}

impl Shader1 {
    pub fn new() -> Rc<Shader1> {
        Rc::new(Self {

        })
    }
}


impl Shader for Shader1 {

    fn vertex_shader(&self, mut core: CorePipe, params: &VertexPipe) -> CorePipe {
        
        core.color = match params.id {
            0 => Some(Vec4::new(255.0, 0.0, 0.0, 255.0)),
            1 => Some(Vec4::new(0.0, 0.0, 255.0, 255.0)),
            _ => Some(Vec4::new(0.0, 255.0, 0.0, 255.0)),
        };

        const X_MOVING_SPEED: f32 = (3.14 * 2.0) * 0.15;

        core.vertex.x += match params.id {
            0 => (params.time.as_secs_f32() * X_MOVING_SPEED * 1.0).sin() * 300.0,
            1 => (params.time.as_secs_f32() * X_MOVING_SPEED * 1.2).sin() * 300.0,
            _ => (params.time.as_secs_f32() * X_MOVING_SPEED * 1.3).sin() * 300.0,
        };

        core.vertex.y = -core.vertex.y + 600.0;

        core
    }

    fn fragment_shader(&self, core: CorePipe, _params: &FragmentPipe) -> Vec4 {
    
        core.color.unwrap()
    }
}