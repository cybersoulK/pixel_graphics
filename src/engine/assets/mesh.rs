use std::io::Read;

use glam::{Vec2, Vec3};

pub struct Mesh {
    vertices: Vec<Vec3>,
    uv_textures: Vec<Vec2>,
    norms: Vec<Vec3>,
    indexes: Vec<[usize; 3]>,

    iterator: usize,
}

impl Mesh { 
    pub fn new(vertices: Vec<Vec3>, uv_textures: Vec<Vec2>, norms: Vec<Vec3>, indexes: Vec<[usize; 3]>) -> Self {
        Self {
            vertices,
            uv_textures,
            norms,
            indexes,

            iterator: 0,
        }
    }

    pub fn build<R: Read>(input: R) -> Self {

        //TODO: use tobj::load

        Self {
            vertices: Vec::new(),
            uv_textures: Vec::new(),
            norms: Vec::new(), 

            indexes: Vec::new(),
            iterator: 0,
        }
    }
}

impl Iterator for Mesh {
    type Item = ([Vec3; 3], [Vec2; 3], [Vec3; 3]);
    
    fn next(&mut self) -> Option<([Vec3; 3], [Vec2; 3], [Vec3; 3])> {

        let mut vertices: [Vec3; 3] = Default::default();
        let mut uv_textures: [Vec2; 3] = Default::default();
        let mut norms:[Vec3; 3] = Default::default();
        
        for i in self.iterator..self.iterator+3 {

            let index = self.indexes[self.iterator];

            vertices[i] = self.vertices[index[0]];
            uv_textures[i] = self.uv_textures[index[1]];
            norms[i] = self.norms[index[2]];

            self.iterator += 1;
        }

        Some((vertices, uv_textures, norms))
    }
}