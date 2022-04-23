use core::panic;
use std::{rc::Rc, collections::HashMap};
use std::fs::File;
use std::io::{BufReader};

pub use model::Model;
pub use mesh::Mesh;
pub use material::Material;
pub use texture::Texture;
pub use shader::Shader;

mod model;
mod mesh;
mod material;
mod texture;
mod shader;


pub struct Assets {
    models: Resource<Model>,
    meshes: Resource<Mesh>,
    materials: Resource<Material>,
    textures: Resource<Texture>,
    shaders: Resource<dyn Shader>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            models: Resource::new(),
            meshes: Resource::new(),
            materials: Resource::new(),
            textures: Resource::new(),
            shaders: Resource::new(),
        }
    }

    pub fn load_model(&mut self, path: &str) -> Rc<Model> {

        if let Some(model) = self.models.get(path) {
            return model;
        }

        else {
            Self::check_path(path);

            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            let model = Model::build(reader, self);
            self.models.set(path, model);

            let get_model = self.models.get(path).unwrap();
            get_model
        }
    }

    pub fn load_mesh(&mut self, path: &str) -> Rc<Mesh> {

        if let Some(mesh) = self.meshes.get(path) {
            return mesh;
        }

        else {
            Self::check_path(path);
        }
    }

    pub fn load_material(&mut self, path: &str) -> Rc<Material> {

        if let Some(material) = self.materials.get(path) {
            return material;
        }

        else {
            Self::check_path(path);
            let file = File::open(path).unwrap();

            let material = Material::build(file, self);
            self.materials.set(path, material);

            let get_material = self.materials.get(path).unwrap();
            get_material
        }
    }

    pub fn load_texture(&mut self, path: &str) -> Rc<Texture> {

        if let Some(texture) = self.textures.get(path) {
            return texture;
        }

        else {
            Self::check_path(path);

            let file = File::open(path).unwrap();

            let texture = Texture::build(file);
            self.textures.set(path, texture);

            let get_texture = self.textures.get(path).unwrap();
            get_texture
        }
    }

    pub fn load_shader(&mut self, path: &str) -> Rc<dyn Shader> {

        if let Some(shader) = self.shaders.get(path) {
            return shader;
        }

        else {
            Self::check_path(path);
        }
    }

    pub fn check_path(path: &str){

        if path.is_empty() { panic!("path must not be empty"); }
        if path.starts_with("#") == true { panic!("path(id) not found: {}", path); }
    }


    pub fn create_model(&mut self, id: &str, model: Rc<Model>){

        Self::check_id(id);

        if self.models.get(id).is_some() {
            panic!("Model created twice!");
        }

        self.models.set(id, model);
    }

    pub fn create_mesh(&mut self, id: &str, mesh: Rc<Mesh>){

        Self::check_id(id);

        if self.meshes.get(id).is_some() {
            panic!("mesh created twice!");
        }

        self.meshes.set(id, mesh);
    }

    pub fn create_material(&mut self, id: &str, material: Rc<Material>){

        Self::check_id(id);

        if self.materials.get(id).is_some() {
            panic!("material created twice!");
        }

        self.material.set(id, material);
    }

    pub fn create_texture(&mut self, id: &str, texture: Rc<Texture>){

        Self::check_id(id);

        if self.textures.get(id).is_some() {
            panic!("texture created twice!");
        }

        self.textures.set(id, texture)
    }

    pub fn create_shader(&mut self, id: &str, shader: Rc<dyn Shader>){

        Self::check_id(id);

        if self.shaders.get(id).is_some() {
            panic!("shader created twice!");
        }

        self.shaders.set(id, shader)
    }

    pub fn check_id(id: &str){

        if id.is_empty() { panic!("id must not be empty"); }
        if id.starts_with("#") == false { panic!("id first character must be '#'"); }
    }
}

struct Resource<R :?Sized> {
    r: HashMap<String, Rc<R>>,
}

impl<R :?Sized> Resource<R> {
    pub fn new() -> Self {
        Self { r: HashMap::new(), } 
    }

    pub fn get(&self, path: &str) -> Option<Rc<R>> {
        
        if let Some(resource) = self.r.get(path) {
            Some(Rc::clone(resource))
        }

        else { return None }
    }

    pub fn set(&mut self, path: &str, r: Rc<R>) {
        self.r.insert(path.to_owned(), r);
    }
}
