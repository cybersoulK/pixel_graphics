use std::{io::Read, rc::Rc};

use super::{Texture, Shader};


pub struct Material {
    textures: Vec<Texture>,
    shader: Rc<dyn Shader>,
}

impl Material {
    pub fn build<R: Read>(input: R, shader: Rc<dyn Shader>) -> Self {

        let textures = Vec::new();

        Self {
            textures,
            shader,
        }
    }
}