use std::rc::Rc;

pub trait Shader {
    fn new();

    fn vertex_shader(&mut self);
    fn fragment_shader(&mut self);
}