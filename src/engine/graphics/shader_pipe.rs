use std::{rc::Rc, time::Duration};

use glam::{Vec2, Vec3, Vec4};

use super::triangles::mul_barycentric;
use super::super::{Light, Material};


#[derive(Clone, Copy)]
pub struct CorePipe {

    pub vertex: Vec3,
    pub uv_mapping: Vec2,
    pub norm: Vec3,

    pub color: Vec4,
}

pub struct VertexPipe {
    
    pub elapsed_time: Duration,
}

pub struct FragmentPipe<'a> {

    pub material: Rc<Material>,
    pub lights: &'a Vec<Light>,
    pub elapsed_time: Duration,
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

    
    fn split_bundle(core_pipe: [CorePipe; 3]) -> ([Vec3; 3], [Vec2; 3], [Vec3; 3], [Vec4; 3]) {

        let vertices = core_pipe.map(|c| c.vertex);
        let uv_mappings = core_pipe.map(|c| c.uv_mapping);
        let norms = core_pipe.map(|c| c.norm);

        let colors = core_pipe.map(|c| c.color);


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