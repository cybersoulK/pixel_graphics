use std::rc::Rc;

use glam::{Vec3, Vec4};

use pixel_graphics::{Shader, CorePipe, VertexPipe, FragmentPipe};


pub struct CustomShader {}

impl CustomShader {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {

        })
    }
}


impl Shader for CustomShader {

    fn vertex_shader(&self, mut core: CorePipe, params: &VertexPipe, _face_id: usize, vertex_id: usize) -> CorePipe {
        
        core.color = match vertex_id {
            0 => Vec4::new(255.0, 0.0, 0.0, 255.0),
            1 => Vec4::new(0.0, 0.0, 255.0, 255.0),
            _ => Vec4::new(0.0, 255.0, 0.0, 255.0),
        };

        const X_MOVING_SPEED: f32 = (3.14 * 2.0) * 0.15;

        core.vertex.x += match vertex_id {
            0 => (params.elapsed_time.as_secs_f32() * X_MOVING_SPEED * 1.0).sin() * 0.1,
            1 => (params.elapsed_time.as_secs_f32() * X_MOVING_SPEED * 1.2).sin() * 0.1,
            _ => (params.elapsed_time.as_secs_f32() * X_MOVING_SPEED * 1.3).sin() * 0.1,
        };

        core
    }

    fn fragment_shader(&self, core: CorePipe, _params: &FragmentPipe, _vertex_2d: Vec3) -> Vec4 {
    
        core.color
    }
}

pub fn build() -> Rc<CustomShader> {
    CustomShader::new()
}