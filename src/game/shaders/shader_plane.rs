use std::{rc::Rc, f32::consts::PI};

use glam::{Vec3, Vec4};

use pixel_graphics::{Shader, CorePipe, VertexPipe, FragmentPipe, Light};


pub struct CustomShader {}

impl CustomShader {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {

        })
    }
}


impl Shader for CustomShader {

    fn vertex_shader(&self, mut core: CorePipe, params: &VertexPipe, mut face_id: usize, vertex_id: usize) -> CorePipe {
        
        face_id %= 12;

        /*core.color = match face_id {
            0 | 1 => Vec4::new(0.71, 0.0, 0.0, 1.0),
            2 | 3 => Vec4::new(0.0, 0.0, 0.78, 1.0),
            4 | 5 => Vec4::new(0.71, 0.71, 0.71, 1.0),
            6 | 7 => Vec4::new(0.0, 0.78, 0.78, 1.0),
            8 | 9 => Vec4::new(0.78, 0.0, 0.78, 1.0),
            _ => Vec4::new(0.71, 0.71, 0.0, 1.0),
        };*/

        //core.color = Vec4::new(0.5294, 0.807843, 0.9216, 1.0);


        core.color.x = (((core.vertex.x - 0.5).powf(2.0) + (core.vertex.z - 0.5).powf(2.0)).sqrt()
                        * params.elapsed_time.as_secs_f32() * FREQUENCY).sin() * 0.6;

        core.color.y = (((core.vertex.x - 0.5).powf(2.0) + (core.vertex.z - 0.5).powf(2.0)).sqrt()
                        * params.elapsed_time.as_secs_f32() * FREQUENCY + PI * 2.0 / 3.0 * 1.0).sin() * 0.6;

        core.color.z = (((core.vertex.x - 0.5).powf(2.0) + (core.vertex.z - 0.5).powf(2.0)).sqrt()
                        * params.elapsed_time.as_secs_f32() * FREQUENCY + PI * 2.0 / 3.0 * 2.0).sin() * 0.6;


        const FREQUENCY: f32 = 4.0;
        const HEIGHT: f32 = 0.2;
        
        core.vertex.y = (((core.vertex.x - 0.5).powf(2.0) + (core.vertex.z - 0.5).powf(2.0)).sqrt()
                        * params.elapsed_time.as_secs_f32() * FREQUENCY).sin() * HEIGHT;

        core.color *= (0.2 * core.vertex.y / HEIGHT) + 0.8;
        
        core
    }

    fn fragment_shader(&self, mut core: CorePipe, params: &FragmentPipe, _vertex_2d: Vec3) -> Vec4 {
        
        let light0 = params.lights.get(0);

        for light in params.lights {
            core.color = spot_lighting(core.color, light);
        }
        
        core.color
    }
}

pub fn spot_lighting(color: Vec4, light: &Light) -> Vec4 {
    
    light.transform;
    color
}

pub fn build() -> Rc<CustomShader> {
    CustomShader::new()
}