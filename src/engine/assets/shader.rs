use glam::{Vec4};

use super::super::{CorePipe, ParamsPipe, DynamicPipe};


#[allow(unused_variables)]
#[allow(unused_mut)]
pub trait Shader {
    fn vertex_shader(&self, mut core: CorePipe, params: &ParamsPipe, dynamic: &mut DynamicPipe, vertex_id: usize, face_id: usize) -> CorePipe
    { core }

    fn geometry_shader(&self, mut core: [CorePipe; 3], params: &ParamsPipe, dynamic: &[&mut DynamicPipe; 3]) -> [CorePipe; 3]
    { core }
    
    fn vertex_world_shader(&self, mut core: CorePipe, params: &ParamsPipe, dynamic: &mut DynamicPipe) -> CorePipe
    { core }

    
    fn fragment_shader(&self, mut core: CorePipe, params: &ParamsPipe, dynamic: DynamicPipe, texture_color: Vec4 ) -> Vec4
    { core.color }
}