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

        let mesh = assets.load_mesh(mesh_path);
        //let material = assets.load_material(texture_path, Rc<>); //TODO: shader has to be imported somehow

        Rc::new(Self {
            meshes,
            materials,
        })
    }
}


impl IntoIterator for Model {
    type Item = (Rc<Mesh>, Rc<Material>);
    type IntoIter = ModelIterator;

    fn into_iter(self) -> Self::IntoIter {
        ModelIterator {
            model: self,
            index: 0,
        }
    }
}

struct ModelIterator {
    model: Model,
    index: usize,
}

impl Iterator for ModelIterator {
    type Item = (Rc<Mesh>, Rc<Material>);
    
    fn next(&mut self) -> Option<(Rc<Mesh>, Rc<Material>)> {
        
        let mesh = self.model.meshes[self.index];
        let material = self.model.materials[mesh.get_material_index()];


        self.index += 1;

        Some((mesh, material))
    }
}