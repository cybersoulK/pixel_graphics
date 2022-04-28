use std::cell::RefCell;
use std::rc::Rc;

use winit::dpi::{PhysicalPosition};
use winit::event_loop::ControlFlow;
use winit::{window::Window};
use winit::event::VirtualKeyCode;


use pixel_graphics::{GameLoop, Component, Engine, InputsManager, Light, Transform};

mod components {
    pub mod camera_movement;
    pub mod simple_movement;
    pub mod trig_component;
    pub mod plane_component;
}
mod meshes {
    pub mod triangle;
    pub mod cube;
    pub mod plane;
}
mod objects {
    pub mod camera;
    pub mod triangle;
    pub mod cube;
    pub mod plane;
}
mod shaders {
    pub mod shader1;
    pub mod shader2;
    pub mod shader_plane;
}


pub struct Game {
    is_exit: bool,
    camera_component: Option<Rc<RefCell<dyn Component>>>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            is_exit: false,
            camera_component: None,
        }
    }
}


impl GameLoop for Game {

    fn init(&mut self, engine: &mut Engine, window: &mut Window){

        window.set_title("our beautiful 3d world");


        let (mut camera, camera_component) = objects::camera::build();
        camera.transform.translation = glam::vec3(0.0, 0.0, -1.0);

        engine.set_camera(camera);
        self.camera_component = Some(camera_component);


        //---------
        let light_transform = Transform {
            translation: glam::vec3(0.0, 0.0, 0.0),
            ..Default::default()
        };
        let light = Light::new(light_transform);
        engine.add_light(light);

        //---------
        let plane = objects::plane::build(engine);
        engine.add_drawable(plane);


        //---------
        /*let triangle = objects::triangle::build(engine);
        engine.add_drawable(triangle);*/
        
        //--------
        /*let mut cube = objects::cube::build(engine);

        cube.transform.translation = glam::vec3(0.0, 0.0, 2.0);
        engine.add_drawable(cube.clone());

        cube.transform.translation = glam::vec3(0.0, 0.0, -2.0);
        engine.add_drawable(cube.clone());

        cube.transform.translation = glam::vec3(2.0, 0.0, 0.0);
        engine.add_drawable(cube.clone());

        cube.transform.translation = glam::vec3(-2.0, 0.0, 0.0);
        engine.add_drawable(cube.clone());*/
        
    }

    fn update_end(&mut self, _engine: &mut Engine, window: &mut Window, inputs: &mut InputsManager){

        update_camera(self.camera_component.as_ref().unwrap(), window, inputs);

        if inputs.is_keyboard_pressed(VirtualKeyCode::Escape) {
            self.is_exit = true;
        }
    }

    fn status(&self, control_flow: &mut ControlFlow) {

        if self.is_exit == false {
            *control_flow = ControlFlow::Poll;
        }
        else {
            *control_flow = ControlFlow::Exit;
        }
    }
}


fn update_camera(camera_component: &RefCell<dyn Component>, window: &mut Window, inputs: &mut InputsManager) {

    let window_size = window.inner_size();

    let center_position = PhysicalPosition::new(
        window_size.width as f64 / 2.0,
        window_size.height as f64 / 2.0);

    
    window.set_cursor_position(center_position).unwrap();
    window.set_cursor_visible(false);

    inputs.cursor_position = center_position;
    

    use components::camera_movement::CameraMovement;

    let mut camera_component = camera_component.borrow_mut();
    let camera_component = camera_component.as_any();
    let camera_component: &mut CameraMovement = camera_component.downcast_mut().unwrap();
    
    camera_component.set_screen_size(window_size);
}