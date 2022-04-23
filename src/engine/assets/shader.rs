use super::super::ShaderPipe;

pub trait Shader {
    fn execute(&self, shader_pipe: &mut ShaderPipe) {
        self.vertex_shader(&mut shader_pipe);
        self.fragment_shader(&mut shader_pipe);
    }

    fn vertex_shader(&self, shader_pipe: &mut ShaderPipe);
    fn fragment_shader(&self, shader_pipe: &mut ShaderPipe);
}