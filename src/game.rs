use pixel_graphics::*;


mod cube;
mod shader1;

pub struct Game {}


impl GameLoop for Game {

    fn init(&mut self, engine: &mut Engine){

        let camera_transform = Transform::default();
        engine.set_camera(camera_transform, 1.0, 1000.0, 90.0);


        let cube_mesh = cube::get_cube();
        let cube_transform = Transform::default();

        let cube_material = Material::new(Vec::);

        let cube_model = Model::new(Vec::new(cube_mesh), Vec::new(cube_material));
        let cube = DrawableObject::new(cube_transform, model);

        engine.add_object(Rc::new(cube));

        let drawable_transform = Transform { 
            position: glam::vec3(100.0, 100.0, 100.0), 
            ..Default::default() 
        };
    }

    fn update(&mut self, engine: &mut Engine){
        
    }
}