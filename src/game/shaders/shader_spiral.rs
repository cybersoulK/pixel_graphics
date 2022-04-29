use std::{rc::Rc, f32::consts::{PI}};

use pixel_graphics::{Shader, CorePipe, ParamsPipe, DynamicPipe};


pub struct CustomShader {}

impl CustomShader {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {

        })
    }
}


impl Shader for CustomShader {

    fn vertex_shader(&self, mut core: CorePipe, params: &ParamsPipe, dynamic: &mut DynamicPipe, mut face_id: usize, vertex_id: usize) -> CorePipe {
        
        //face_id %= 12;

        /*core.color = match face_id {
            0 | 1 => Vec4::new(0.71, 0.0, 0.0, 1.0),
            2 | 3 => Vec4::new(0.0, 0.0, 0.78, 1.0),
            4 | 5 => Vec4::new(0.71, 0.71, 0.71, 1.0),
            6 | 7 => Vec4::new(0.0, 0.78, 0.78, 1.0),
            8 | 9 => Vec4::new(0.78, 0.0, 0.78, 1.0),
            _ => Vec4::new(0.71, 0.71, 0.0, 1.0),
        };*/

        //core.color = Vec4::new(0.5294, 0.807843, 0.9216, 1.0);]

        fn get_angle(vec: glam::Vec2) -> f32 {
        
            let radius = (vec.x.powf(2.0) + vec.y.powf(2.0)).sqrt();
            let mut angle;
    
            if vec.x > 0.0 && vec.y > 0.0 {
    
                let sin = vec.y / radius;
                angle = sin.asin();
            }
            else {
                
                let cos = vec.x / radius;
                angle = cos.acos();
    
                if vec.y < 0.0 { angle = PI * 2.0 - angle}
            }
    
            angle
        }
                        
        const FREQUENCY: f32 = 2.0;
        const HEIGHT: f32 = 1.0;

        let x = core.vertex.x - 0.5;
        let z = core.vertex.z - 0.5;

        let spiral = (get_angle(glam::vec2(x, z)) + (PI * 2.0) * (x.powf(2.0) + z.powf(2.0)).sqrt() * params.elapsed_time.as_secs_f32() * FREQUENCY ).sin();
        core.vertex.y = spiral * HEIGHT + (1.0 / ((x.powf(2.0) + z.powf(2.0)).sqrt() + 0.001)) * 0.2;

        core.color.x = 0.3;
        core.color.z = 0.5;

        core.color.y = spiral * 0.8;

        core.color *= (0.2 * core.vertex.y / HEIGHT) + 0.8;
        
        core
    }
}


pub fn build() -> Rc<CustomShader> {
    CustomShader::new()
}