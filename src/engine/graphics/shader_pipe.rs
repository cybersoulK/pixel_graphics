use std::{rc::Rc, time::Duration};

use glam::{Vec2, Vec3, Vec4};

use super::triangles::mul_barycentric;
use super::super::{Camera, Light, Material};


#[derive(Clone, Copy)]
pub struct CorePipe {

    pub vertex: Vec3,
    pub uv_mapping: Vec2,
    pub norm: Vec3,


    pub color: Vec4,
}

pub struct ParamsPipe<'a> {
    
    pub elapsed_time: Duration,

    pub camera: &'a Camera,

    pub material: Rc<Material>,
    pub lights: &'a Vec<Light>,
}

pub struct DynamicPipe {
    
    float_array: Vec<f32>,
}



impl CorePipe {
    pub fn new_bundle(vertices: [Vec3; 3], uv_mappings: [Vec2; 3], norms: [Vec3; 3], colors: [Vec4; 3]) -> [CorePipe; 3] {

        [0, 1, 2].map(|i| {

            CorePipe {
                vertex: vertices[i],
                uv_mapping: uv_mappings[i],
                norm: norms[i],

                color: colors[i],
            }
        })
    }

    
    pub fn split_bundle(core_pipe: [CorePipe; 3]) -> ([Vec3; 3], [Vec2; 3], [Vec3; 3], [Vec4; 3]) {

        let vertices = core_pipe.map(|p| p.vertex);
        let uv_mappings = core_pipe.map(|p| p.uv_mapping);
        let norms = core_pipe.map(|p| p.norm);

        let colors = core_pipe.map(|p| p.color);


        (vertices, uv_mappings, norms, colors)
    }

    pub fn mul_barycentric(core_pipe: [CorePipe; 3], weights: [f32; 3]) -> Self {

        let (vertices, uv_mappings, norms, colors) = Self::split_bundle(core_pipe);

        Self {
            vertex: mul_barycentric(weights, vertices),
            uv_mapping: mul_barycentric(weights, uv_mappings),
            norm: mul_barycentric(weights, norms),

            color: mul_barycentric(weights, colors),
        }
    }
}


impl DynamicPipe {

    pub fn new() -> Self {
        Self {
            float_array: Vec::new(),
        }
    }


    pub fn push(&mut self, key: usize, value: f32) {

        if key >= self.float_array.len()  {
            self.float_array.resize(key + 1, -1.0);
        }

        self.float_array[key] = value;
    }

    pub fn get(&self, key: usize) -> f32 {
        self.float_array[key]
    }

    pub fn len(&self) -> usize {
        self.float_array.len()
    }


    pub fn mul_barycentric(dynamic_pipe: &[&mut DynamicPipe; 3], weights: [f32; 3]) -> Self {

        let mut intersected_dynamic_pipe = DynamicPipe::new();

        for i in 0..dynamic_pipe[0].len() {

            let values = [0, 1, 2].map(|i| {
                dynamic_pipe[i].get(i)
            });

            let intersected = mul_barycentric(weights, values);
            intersected_dynamic_pipe.push(i, intersected);
        }
        
        intersected_dynamic_pipe
    }
}