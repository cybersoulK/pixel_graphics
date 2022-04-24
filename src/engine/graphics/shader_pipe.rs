use std::{rc::Rc};

use glam::{Vec2, Vec3, Vec4, Mat4};

use super::super::{Light, Material};


#[derive(Clone, Copy)]
pub struct VertexPipe {

    pub id: usize,

    pub vertex: Vec3,
    pub uv_mapping: Vec2,
    pub norm: Vec3,

    pub color: Option<Vec4>,

    pub mvp_matrix: Mat4,
}

pub struct FragmentPipe<'a> {

    pub vertex: Vec3,
    pub uv_mapping: Vec2,
    pub norm: Vec3,

    pub color: Option<Vec4>,

    pub material: Rc<Material>,
    pub lights: &'a Vec<Rc<Light>>,
}



impl VertexPipe {
    pub fn transform_option_bundle<T>(array: [Option<T>; 3]) -> Option<[T; 3]> {

        for t in &array {
            if t.is_none() { return None; }
        }

        let unwraped_array = array.map(|t| {
            t.unwrap()
        });

        Some(unwraped_array)
    }
}