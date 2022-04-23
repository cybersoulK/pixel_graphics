use std::{io::Read, rc::Rc};

use glam::{Vec2, Vec3};

pub struct Mesh {
    vertices: Vec<Vec3>,
    uv_textures: Vec<Vec2>,
    norms: Vec<Vec3>,

    indexes: Vec<[usize; 3]>,
    material_index: usize,
}

impl Mesh { 
    pub fn new(vertices: Vec<Vec3>, uv_textures: Vec<Vec2>, norms: Vec<Vec3>, indexes: Vec<[usize; 3]>, material_index: usize) -> Rc<Self> {
        Rc::new(Self {
            vertices,
            uv_textures,
            norms,

            indexes,
            material_index,
        })
    }

    pub fn build<R: Read>(input: R) -> Rc<Self> {

        //TODO: use tobj::load

        Rc::new(Self {
            vertices: Vec::new(),
            uv_textures: Vec::new(),
            norms: Vec::new(), 

            indexes: Vec::new(),
            material_index: 0
        })
    }

    pub fn get_material_index(&self) -> usize {
        self.material_index
    }
}


pub struct MeshIterator {
    mesh: Rc<Mesh>,
    index: usize,
}

impl MeshIterator {
    pub fn new(mesh: &Rc<Mesh>) -> Self {
        Self {
            mesh: Rc::clone(mesh),
            index: 0,
        }
    }
}

impl Iterator for MeshIterator {
    type Item = ([Vec3; 3], [Vec2; 3], [Vec3; 3]);
    
    fn next(&mut self) -> Option<([Vec3; 3], [Vec2; 3], [Vec3; 3])> {

        let mut vertices: [Vec3; 3] = Default::default();
        let mut uv_textures: [Vec2; 3] = Default::default();
        let mut norms:[Vec3; 3] = Default::default();
        
        for i in self.index..self.index+3 {

            let mesh_index = self.mesh.indexes[self.index];

            vertices[i] = self.mesh.vertices[mesh_index[0]];
            uv_textures[i] = self.mesh.uv_textures[mesh_index[1]];
            norms[i] = self.mesh.norms[mesh_index[2]];

            self.index += 1;
        }

        Some((vertices, uv_textures, norms))
    }
}