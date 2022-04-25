use glam::{Vec3, Vec4};

use super::super::{CorePipe, VertexPipe, FragmentPipe};

pub trait Shader {
    fn vertex_shader(&self, core: CorePipe, params: &VertexPipe, vertex_id: usize, face_id: usize) -> CorePipe;
    fn fragment_shader(&self, core: CorePipe, params: &FragmentPipe, vertex_2d: Vec3) -> Vec4;
}