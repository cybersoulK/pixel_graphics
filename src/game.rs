use std::{f32::consts::PI};

use pixel_graphics::*;


mod triangle;
mod cube;
mod shader1;
mod shader2;

mod camera_movement;
mod trig_component;
mod cube_component;


pub struct Game {}


impl GameLoop for Game {

    fn init(&mut self, engine: &mut Engine){

        let mut camera_transform = Transform::default();
        camera_transform.translation.z = -1000.0;
        camera_transform.translation.y = 0.0;
        camera_transform.translation.x = 0.0;
        engine.set_camera(camera_transform, 0.01, (45.0 / 90.0) * (PI / 2.0));


        
        let cube_mesh = cube::get_cube();
        let cube_transform = Transform { 
            translation: glam::vec3(0.0, 0.0, 0.0), 
            scale: glam::vec3(3.3, 2.2, 1.1), 
            rotation: glam::vec3(1.0, 2.0, 3.0),
            ..Default::default() 
        };

        let shader2 = shader2::Shader2::new();
        engine.assets.create_shader("#shader2", shader2);

        let cube_shader = engine.assets.load_shader("#shader2");
        /*let cube_texture = engine.assets.load_texture("...");*/  let cube_texture = Default::default();
        let cube_material = Material::new([cube_texture].to_vec(), cube_shader);

        let cube_model = Model::new([cube_mesh].to_vec(), [cube_material].to_vec());
        let mut cube = DrawableObject::new(cube_transform, cube_model);

        let mut cube_component = cube_component::TestComponent {};
        cube.components.add(cube_component);

        engine.add_drawable(cube);
        

/* 
        let trig_mesh = triangle::get_triangle();
        let trig_transform = Transform { 
            translation: glam::vec3(0.0, 0.0, 0.0), 
            scale: glam::vec3(2.0, 2.0, 1.0), 
            ..Default::default() 
        };
        
        let shader1 = shader1::Shader1::new();
        engine.assets.create_shader("#shader1", shader1);

        let trig_shader = engine.assets.load_shader("#shader1");
        /*let trig_texture = engine.assets.load_texture("...");*/  let trig_texture = Default::default();
        let trig_material = Material::new([trig_texture].to_vec(), trig_shader);

        let trig_model = Model::new([trig_mesh].to_vec(), [trig_material].to_vec());
        let mut trig = DrawableObject::new(trig_transform, trig_model);

        let mut trig_component = test_component::TestComponent {};
        trig.components.add(trig_component);
        

        engine.add_drawable(trig.clone());*/
    }

    fn update(&mut self, engine: &mut Engine){
        
    }
}