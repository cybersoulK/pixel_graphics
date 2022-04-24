use glam::Vec4;

use super::super::{VertexPipe, FragmentPipe};

pub trait Shader {
    fn vertex_shader(&self, inputs: VertexPipe) -> VertexPipe;
    fn fragment_shader(&self, inputs: FragmentPipe) -> Vec4;
}