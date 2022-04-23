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

        let model = self.models.get(path);

        if let Some(model) = model {
            return model;
        }

        else {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            let model = Model::build(reader, self);
            self.models.set(path, Rc::new(model));

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
            self.meshes.set(path, Rc::new(mesh));

            let get_mesh = self.meshes.get(path).unwrap();
            get_mesh
        }
    }

    pub fn load_material(&mut self, path: &str, shader: Option<Rc<dyn Shader>>) -> Rc<Material> {

        let material = self.materials.get(path);

        if let Some(material) = material {
            return material;
        }

        else {
            let file = File::open(path).unwrap();

            let material = Material::build(file, shader.expect("Shader is None"));
            self.materials.set(path, Rc::new(material));

            let get_texture = self.materials.get(path).unwrap();
            get_texture
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
            self.textures.set(path, Rc::new(texture));

            let get_texture = self.textures.get(path).unwrap();
            get_texture
        }
    }


    pub fn create_model(&mut self, id: &str, model: Model){

        if id.is_empty() { panic!("id must not be empty"); }
        if id.starts_with("#") == false { panic!("id first character must be '#'"); }


        let model = self.models.get(id);

        if let Some(model) = model {
            panic!("Model created twice!");
        }


    }

    pub fn check_id(id: &str){
        
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
