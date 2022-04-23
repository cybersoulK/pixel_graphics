use std::rc::Rc;
use std::io::Read;

use super::Assets;
use super::{Mesh, Material};


pub struct Model {
    meshes: Vec<Rc<Mesh>>,
    materials: Vec<Rc<Material>>,

    material_index: Vec<usize>,
}

impl Model {
    pub fn new(meshes: Vec<Rc<Mesh>>, materials: Vec<Rc<Material>>, material_index: Vec<usize>) -> Self {

        Self {
            meshes: Vec::new(),
            materials: Vec::new(),

            material_index,
        }
    }

    pub fn build<R: Read>(input: R, assets: &mut Assets) -> Self {

        let mesh_path = "";
        let texture_path = "";

        let meshes = Vec::new();
        let materials = Vec::new();
        let material_index = Vec::new();

        let mesh = assets.load_mesh(mesh_path);
        //let material = assets.load_material(texture_path, Rc<>); //TODO: shader has to be imported somehow

        Self {
            meshes,
            materials,

            material_index,
        }
    }
}