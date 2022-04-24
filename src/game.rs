use std::rc::Rc;

use pixel_graphics::*;


mod triangle;
mod cube;
mod shader1;

pub struct Game {}


impl GameLoop for Game {

    fn init(&mut self, engine: &mut Engine){

        let camera_transform = Transform::default();
        engine.set_camera(camera_transform, 1.0, 1000.0, 90.0);


        /*
        let cube_mesh = cube::get_cube();
        let cube_transform = Transform { 
            translation: glam::vec3(100.0, 100.0, 100.0), 
            scale: glam::vec3(3.0, 2.0, 1.0), 
            ..Default::default() 
        };

        let shader1 = shader1::Shader1::new();
        engine.assets.create_shader("#shader1", shader1);

        let cube_shader = engine.assets.load_shader("#shader1");
        /*let cube_texture = engine.assets.load_texture("...");*/  let cube_texture = Default::default();
        let cube_material = Material::new([cube_texture].to_vec(), cube_shader);

        let cube_model = Model::new([cube_mesh].to_vec(), [cube_material].to_vec());
        let cube = DrawableObject::new(cube_transform, cube_model);

        engine.add_drawable(Rc::new(cube));
        */


        let trig_mesh = triangle::get_triangle();
        let trig_transform = Transform { 
            translation: glam::vec3(100.0, 100.0, 100.0), 
            scale: glam::vec3(3.0, 2.0, 1.0), 
            ..Default::default() 
        };
        
        let shader1 = shader1::Shader1::new();
        engine.assets.create_shader("#shader1", shader1);

        let trig_shader = engine.assets.load_shader("#shader1");
        /*let trig_texture = engine.assets.load_texture("...");*/  let trig_texture = Default::default();
        let trig_material = Material::new([trig_texture].to_vec(), trig_shader);

        let trig_model = Model::new([trig_mesh].to_vec(), [trig_material].to_vec());
        let trig = DrawableObject::new(trig_transform, trig_model);

        engine.add_drawable(Rc::new(trig.clone()));
    }

    fn update(&mut self, engine: &mut Engine){
        
    }
}