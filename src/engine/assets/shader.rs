use std::rc::Rc;

use super::Material;

pub trait Shader {
    fn vertex_shader(&mut self, material: Rc<Material>);
    fn fragment_shader(&mut self);
}