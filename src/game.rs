use pixel_graphics::*;


mod cube;
mod shader1;

pub struct Game {}


impl GameLoop for Game {

    fn init(&mut self, engine: &mut Engine){

        let camera_transform = Transform::default();
        engine.set_camera(camera_transform, 1.0, 1000.0, 90.0);


        let cube_mesh = cube::get_cube();
        let cube_transform = Transform { 
            position: glam::vec3(100.0, 100.0, 100.0), 
            scale: glam::vec3(3.0, 2.0, 1.0), 
            ..Default::default() 
        };

        let cube_texture = engine.assets.load_texture("");

        let shader1 = shader1::Shader1::new();
        engine.assets.create_shader("#shader1", cube_shader);


        let cube_shader = engine.assets.load_shader("#shader1");
        let cube_material = Material::new([cube_texture], cube_shader);

        let cube_model = Model::new([cube_mesh], [cube_material]);
        let cube = DrawableObject::new(cube_transform, model);

        engine.add_object(Rc::new(cube));

    }

    fn update(&mut self, engine: &mut Engine){
        
    }
}