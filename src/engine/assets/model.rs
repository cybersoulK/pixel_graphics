use std::rc::Rc;
use std::io::Read;

use super::Assets;
use super::{Mesh, Material};


pub struct Model {
    meshes: Vec<Rc<Mesh>>,
    materials: Vec<Rc<Material>>,
}

impl Model {
    pub fn new(meshes: Vec<Rc<Mesh>>, materials: Vec<Rc<Material>>) -> Rc<Self> {

        Rc::new(Self {
            meshes: Vec::new(),
            materials: Vec::new(),
        })
    }

    pub fn build<R: Read>(input: R, assets: &Assets) -> Rc<Self> {

        let mesh_path = "";
        let texture_path = "";

        let meshes = Vec::new();
        let materials = Vec::new();
        let material_index = Vec::new();

        let mesh = assets.load_mesh(mesh_path);
        //let material = assets.load_material(texture_path, Rc<>); //TODO: shader has to be imported somehow

        Rc::new(Self {
            meshes,
            materials,
        })
    }
}