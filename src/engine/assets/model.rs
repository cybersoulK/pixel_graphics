use std::rc::Rc;
use std::io::Read;

use super::Assets;
use super::{Mesh, Material};


pub struct Model {
    meshes: Rc<Mesh>,
    //material: Rc<Material>,
}

impl Model {
    pub fn build<R: Read>(input: R, assets: &mut Assets) -> Self {

        let mesh_path = "";
        let texture_path = "";

        let meshes = assets.load_mesh(mesh_path);
        //let material = assets.load_material(texture_path, Rc<>); //TODO: shader has to be imported somehow

        Self {
            meshes,
            //material,
        }
    }
}