pub use std::rc::Rc;

pub use super::Material;
pub use glam::vec3;

pub trait Shader {
    fn vertex_shader(&self, material: Rc<Material>);
    fn fragment_shader(&self);
}