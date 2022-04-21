use std::io::Read;


pub struct Mesh {
    vertices: Vec<glam::Vec3>,
    norms: Vec<glam::Vec3>,

    indexes: Vec<usize>,
}

impl Mesh {
    pub fn build<R: Read>(input: R) -> Self {
        Self {
            vertices: Vec::new(),
            norms: Vec::new(),

            indexes: Vec::new(),
        }
    }
}