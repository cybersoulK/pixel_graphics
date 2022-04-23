use std::{io::Read, rc::Rc};

use super::{Assets, Texture, Shader};


pub struct Material {
    textures: Vec<Rc<Texture>>,
    shader: Rc<dyn Shader>,
}

impl Material {
    pub fn new(textures: Vec<Rc<Texture>>, shader: Rc<dyn Shader>) -> Rc<Self> {
        Rc::new(Self {
            textures,
            shader,
        })
    }
    pub fn build<R: Read>(input: R, assets: &mut Assets) -> Rc<Self> {

        let textures = Vec::new();
        let shader = assets.load_shader("");

        Rc::new(Self {
            textures,
            shader,
        })
    }

    pub fn get_shader(&self) -> Rc<dyn Shader> {
        Rc::clone(&self.shader)
    }
}