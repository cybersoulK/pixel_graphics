use pixel_graphics::GameLoop;
use pixel_graphics::Engine;

use pixel_graphics::assets::Model;

use pixel_graphics::objects::Camera;
use pixel_graphics::objects::DrawableObject;
use pixel_graphics::objects::Transform;

pub struct Game {}


impl GameLoop for Game {

    fn init(&mut self, engine: &mut Engine){

        let camera_transform = Transform::default();
        
        let main_camera = Camera::new(camera_transform, 1.0, 1000.0, 90.0);

        //engine.addObject(main_camera);
        //engine.setCamera(main_camera);


        let drawable_transform = Transform { 
            position: glam::vec3(100.0, 100.0, 100.0), 
            ..Default::default() 
        };
        let drawable_model = Model {};
        let drawable_object = DrawableObject::new(drawable_transform, drawable_model);
        
        //engine.addObject(drawable_object);
    }

    fn update(&mut self, engine: &mut Engine){
        
    }
}