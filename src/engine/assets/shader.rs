pub trait Shader {
    fn vertex_shader(&mut self);
    fn fragment_shader(&mut self);
}