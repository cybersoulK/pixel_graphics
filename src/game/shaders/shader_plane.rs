use std::{rc::Rc, f32::consts::{PI}};

use glam::{Vec3};

use pixel_graphics::{Shader, CorePipe, ParamsPipe, DynamicPipe};

use super::common_fn::{directional_light, spot_lighting};


pub struct CustomShader {}

impl CustomShader {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {

        })
    }
}

const DIRECTIONAL_LIGHT: usize = 0;
const SPOT_LIGHT: usize = 1;

impl Shader for CustomShader {

    fn vertex_shader(&self, mut core: CorePipe, params: &ParamsPipe, dynamic: &mut DynamicPipe, mut face_id: usize, vertex_id: usize) -> CorePipe {
        
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
                        
        const FREQUENCY: f32 = 0.2;
        const HEIGHT: f32 = 1.0;

        let x = core.vertex.x - 0.5;
        let z = core.vertex.z - 0.5;

        let spiral = (get_angle(glam::vec2(x, z)) + (PI * 2.0) * (x.powf(2.0) + z.powf(2.0)).sqrt() * params.elapsed_time.as_secs_f32() * FREQUENCY ).sin();
        core.vertex.y = spiral * HEIGHT + (1.0 / ((x.powf(2.0) + z.powf(2.0)).sqrt() + 0.001)) * 0.2;

        core.color.x = 0.3;
        core.color.y = 0.5;
        core.color.z = 0.8;

        core
    }

    fn geometry_shader(&self, mut core: [CorePipe; 3], params: &ParamsPipe, dynamic: &[&mut DynamicPipe; 3]) -> [CorePipe; 3] {
        
        let vec1 = core[0].vertex - core[1].vertex;
        let vec2 = core[2].vertex - core[1].vertex;

        let cross = vec1.cross(vec2).normalize();


        core[0].norm = cross;
        core[1].norm = cross;
        core[2].norm = cross;

        core
    }

    fn vertex_world_shader(&self, mut core: CorePipe, params: &ParamsPipe, dynamic: &mut DynamicPipe) -> CorePipe {
        
        let mut spot_light = 0.0;

        for light in params.lights {
            spot_light += spot_lighting(core.vertex, core.norm, light);
        }

        let directional_light = directional_light(core.norm, Vec3::new(1.0, -1.0, 1.0));
        

        dynamic.push(DIRECTIONAL_LIGHT, directional_light); 
        dynamic.push(SPOT_LIGHT, spot_light); 

        core
    }


    fn fragment_shader(&self, mut core: CorePipe, params: &ParamsPipe, dynamic: DynamicPipe, texture_color: glam::Vec4) -> glam::Vec4 {
        
        let directional_light = dynamic.get(DIRECTIONAL_LIGHT);
        let spot_light = dynamic.get(SPOT_LIGHT);

        let mut color = core.color * (directional_light);
        color.w = 1.0;
        
        color
    }
}

pub fn build() -> Rc<CustomShader> {
    CustomShader::new()
}