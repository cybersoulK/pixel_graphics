use pixel_graphics::{Engine, Transform, DrawableObject, Material, Model};

use crate::game::{shaders, meshes, components};


pub fn build(engine: &mut Engine) -> DrawableObject {

    let plane_mesh = meshes::plane::build((301, 301));

    let plane_transform = Transform { 
        translation: glam::vec3(-0.5, -1.0, 0.5), 
        scale: glam::vec3(5.0, 0.0, 5.0), 
        ..Default::default() 
    };

    let plane_shader = shaders::shader_plane::build();
    engine.assets.create_shader("#shader3", plane_shader);

    let plane_shader = engine.assets.load_shader("#shader3");
    let plane_texture = Default::default();
    let plane_material = Material::new([plane_texture].to_vec(), plane_shader);

    let plane_model = Model::new([plane_mesh].to_vec(), [plane_material].to_vec());
    let mut plane = DrawableObject::new(plane_transform, plane_model);

    let plane_component = components::plane_component::build();
    plane.components.add(plane_component);

    plane
}