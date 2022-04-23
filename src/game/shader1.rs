use super::shader::*;


pub struct Shader1 {

}

impl Shader1 {
    pub fn new() -> Rc<Shader1> {
        Rc::new(Self {

        })
    }
}

impl Shader for Shader1 {
    fn vertex_shader(&self, material: Rc<Material>) {

    }

    fn fragment_shader(&self) {

    }
}