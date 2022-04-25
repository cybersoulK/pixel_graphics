use glam::Vec4;

use super::super::{CorePipe, VertexPipe, FragmentPipe};

pub trait Shader {
    fn vertex_shader(&self, vertex_id: usize, core: CorePipe, params: &VertexPipe) -> CorePipe;
    fn fragment_shader(&self, core: CorePipe, params: &FragmentPipe) -> Vec4;
}