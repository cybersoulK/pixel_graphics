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
            meshes,
            materials,
        })
    }

    pub fn build<R: Read>(input: R, assets: &mut Assets) -> Rc<Self> {

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


pub struct ModelIterator {
    model: Rc<Model>,
    index: usize,
}

impl ModelIterator {
    pub fn new(model: &Rc<Model>) -> Self {
        Self {
            model: Rc::clone(model),
            index: 0,
        }
    }
}

impl Iterator for ModelIterator {
    type Item = (Rc<Mesh>, Rc<Material>);
    
    fn next(&mut self) -> Option<(Rc<Mesh>, Rc<Material>)> {
        
        if self.index >= self.model.meshes.len() { return None; }


        let mesh = Rc::clone(&self.model.meshes[self.index]);
        let material = Rc::clone(&self.model.materials[mesh.get_material_index()]);

        
        self.index += 1;
        
        Some((mesh, material)) //TODO: has to be cloned. want to check compile error
    }
}