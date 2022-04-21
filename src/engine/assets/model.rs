use std::rc::Rc;
use std::io::Read;

use super::Assets;
use super::{Mesh, Texture};


pub struct Model {
    mesh: Rc<Mesh>,
    texture: Rc<Texture>,
}

impl Model {
    pub fn build<R: Read>(input: R, assets: &mut Assets) -> Self {

        let mesh_path = "";
        let texture_path = "";

        let mesh = assets.load_mesh(mesh_path);
        let texture = assets.load_texture(texture_path);

        Self {
            mesh,
            texture,
        }
    }
}