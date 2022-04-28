use pixel_graphics::{Engine, Transform, DrawableObject, Material, Model};

use crate::game::{shaders, meshes, components};


pub fn build(engine: &mut Engine) -> DrawableObject {

    let trig_mesh = meshes::triangle::build();
    let trig_transform = Transform { 
        translation: glam::vec3(0.0, 0.0, 0.0), 
        scale: glam::vec3(3.0, 2.0, 1.0), 
        ..Default::default() 
    };
    
    let shader1 = shaders::shader1::build();
    engine.assets.create_shader("#shader1", shader1);

    let trig_shader = engine.assets.load_shader("#shader1");
    /*let trig_texture = engine.assets.load_texture("...");*/  let trig_texture = Default::default();
    let trig_material = Material::new([trig_texture].to_vec(), trig_shader);

    let trig_model = Model::new([trig_mesh].to_vec(), [trig_material].to_vec());
    let mut trig = DrawableObject::new(trig_transform, trig_model);

    let mut trig_component = components::trig_component::build();
    trig.components.add(trig_component);

    trig
}