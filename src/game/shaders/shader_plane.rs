use std::{rc::Rc, f32::consts::{PI, E}};

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
                        
        const FREQUENCY: f32 = 1.0;
        const HEIGHT: f32 = 1.0;

        let x = core.vertex.x - 0.5;
        let z = core.vertex.z - 0.5;

        let a = 1.0 + params.elapsed_time.as_secs_f32() * FREQUENCY;
        let b = 1.0 + params.elapsed_time.as_secs_f32() * FREQUENCY;
        let c = 2.0 + params.elapsed_time.as_secs_f32() * FREQUENCY;

        //
        let spiral = ((x.powf(2.0) + z.powf(2.0) * a).sqrt().tan() * x * b - z * c);
        let spiral_norm = -spiral.powf(2.0) + 1.0;
        core.vertex.y = spiral_norm * HEIGHT;

        core.color.x = 0.3;
        core.color.z = 0.5;

        core.color.y = spiral * 0.8 + 0.2;

        core.color *= (0.5 * core.vertex.y / HEIGHT) + 0.5;
        
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