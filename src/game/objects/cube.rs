use pixel_graphics::{Transform, DrawableObject, Material, Model, Assets};

use crate::game::{shaders, meshes, components};


pub fn build(assets: &mut Assets) -> DrawableObject {

    let cube_mesh = meshes::cube::build();
    let cube_transform = Transform { 
        scale: glam::vec3(0.3, 0.2, 0.1), 
        ..Default::default() 
    };

    let shader2 = shaders::shader2::build();
    assets.create_shader("#shader2", shader2);

    let cube_shader = assets.load_shader("#shader2");
    /*let cube_texture = engine.assets.load_texture("...");*/  let cube_texture = Default::default();
    let cube_material = Material::new([cube_texture].to_vec(), cube_shader);

    let cube_model = Model::new([cube_mesh].to_vec(), [cube_material].to_vec());
    let mut cube = DrawableObject::new(cube_transform, cube_model);

    let cube_component = components::simple_movement::build();
    cube.components.add(cube_component);

    cube
}