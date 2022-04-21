use std::{rc::Rc, collections::HashMap};
use std::fs::File;
use std::io::{BufReader};

pub use model::Model;
pub use mesh::Mesh;
pub use texture::Texture;

mod model;
mod mesh;
mod texture;


pub struct Assets {
    models: Resource<Model>,
    meshes: Resource<Mesh>,
    textures: Resource<Texture>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            models: Resource::new(),
            meshes: Resource::new(),
            textures: Resource::new(),
        }
    }

    pub fn load_model(&mut self, path: &str) -> Rc<Model> {

        let model = self.models.get(path);

        if let Some(model) = model {
            return model;
        }

        else {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            let model = Model::build(reader, self);
            self.models.set(path, model);

            let get_model = self.models.get(path).unwrap();
            get_model
        }
    }

    pub fn load_mesh(&mut self, path: &str) -> Rc<Mesh> {

        let mesh = self.meshes.get(path);

        if let Some(mesh) = mesh {
            return mesh;
        }

        else {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            let mesh = Mesh::build(reader);
            self.meshes.set(path, mesh);

            let get_mesh = self.meshes.get(path).unwrap();
            get_mesh
        }
    }

    pub fn load_texture(&mut self, path: &str) -> Rc<Texture> {

        let texture = self.textures.get(path);

        if let Some(texture) = texture {
            return texture;
        }

        else {
            let file = File::open(path).unwrap();

            let texture = Texture::build(file);
            self.textures.set(path, texture);

            let get_texture = self.textures.get(path).unwrap();
            get_texture
        }
    }

}

struct Resource<R> {
    r: HashMap<String, Rc<R>>,
}

impl<R> Resource<R> {
    pub fn new() -> Self {
        Self { r: HashMap::new(), } 
    }

    pub fn get(&self, path: &str) -> Option<Rc<R>> {
        
        if let Some(resource) = self.r.get(path) {
            Some(Rc::clone(resource))
        }

        else { return None }
    }

    pub fn set(&mut self, path: &str, r: R) {
        self.r.insert(path.to_owned(), Rc::new(r));
    }
}
