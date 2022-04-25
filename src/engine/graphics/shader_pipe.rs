use std::{rc::Rc, time::Duration};

use glam::{Vec2, Vec3, Vec4, Mat4};

use super::super::{Light, Material};


#[derive(Clone, Copy)]
pub struct CorePipe {

    pub vertex: Vec3,
    pub uv_mapping: Vec2,
    pub norm: Vec3,

    pub color: Option<Vec4>,
}

pub struct VertexPipe {
    
    pub time: Duration,
}

pub struct FragmentPipe<'a> {

    pub material: Rc<Material>,
    pub lights: &'a Vec<Light>,
    pub time: Duration,
}



impl CorePipe {
    pub fn new_bundle(vertices: [Vec3; 3], uv_mappings: [Vec2; 3], norms: [Vec3; 3], colors: [Option<Vec4>; 3]) -> [CorePipe; 3] {

        [0, 1, 2].map(|i| {

            CorePipe {
                vertex: vertices[i],
                uv_mapping: uv_mappings[i],
                norm: norms[i],

                color: colors[i],
            }
        })
    }

    pub fn transform_option<T>(array: [Option<T>; 3]) -> Option<[T; 3]> {

        for t in &array {
            if t.is_none() { return None; }
        }

        let unwraped_array = array.map(|t| {
            t.unwrap()
        });

        Some(unwraped_array)
    }
}