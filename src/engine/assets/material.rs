use std::{io::Read, rc::Rc};

use super::{Texture, Shader};


pub struct Material {
    textures: Vec<Rc<Texture>>,
    shader: Rc<dyn Shader>,
}

impl Material {
    pub fn new(textures: Vec<Rc<Texture>>, shader: Rc<dyn Shader>) -> Self{
        Self {
            textures,
            shader,
        }
    }
    pub fn build<R: Read>(input: R, shader: Rc<dyn Shader>) -> Self {

        let textures = Vec::new();

        Self {
            textures,
            shader,
        }
    }
}