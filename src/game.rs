use std::cell::RefCell;
use std::rc::Rc;
use std::{f32::consts::PI};

use glam::EulerRot;
use winit::dpi::{PhysicalPosition};
use winit::event_loop::ControlFlow;
use winit::{window::Window};
use winit::event::VirtualKeyCode;

use pixel_graphics::*;

use camera_movement::CameraMovement;


mod triangle;
mod cube;
mod shader1;
mod shader2;

mod camera_movement;
mod trig_component;
mod cube_component;


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

        
        let mut camera_transform = Transform::default();
        camera_transform.translation.z = -1.5;
        camera_transform.translation.y = 0.0;
        camera_transform.translation.x = 0.0;

        let mut camera = Camera::new(camera_transform, 0.01, (70.0 / 90.0) * (PI / 2.0));
        

        let camera_component = CameraMovement::new();
        let camera_component = camera.components().add(camera_component);

        self.camera_component = Some(camera_component);

        engine.set_camera(camera);

        
        let cube_mesh = cube::get_cube();
        let cube_transform = Transform { 
            translation: glam::vec3(0.0, 0.0, 0.0), 
            scale: glam::vec3(0.3, 0.2, 0.1), 
            rotation: glam::vec3(0.0, 0.0, 0.0),
            ..Default::default() 
        };

        let shader2 = shader2::Shader2::new();
        engine.assets.create_shader("#shader2", shader2);

        let cube_shader = engine.assets.load_shader("#shader2");
        /*let cube_texture = engine.assets.load_texture("...");*/  let cube_texture = Default::default();
        let cube_material = Material::new([cube_texture].to_vec(), cube_shader);

        let cube_model = Model::new([cube_mesh].to_vec(), [cube_material].to_vec());
        let mut cube = DrawableObject::new(cube_transform, cube_model);

        let cube_component = cube_component::TestComponent {};
        cube.components.add(cube_component);

        engine.add_drawable(cube);
    }

    fn update_end(&mut self, _engine: &mut Engine, window: &mut Window, inputs: &mut InputsManager){


        let window_size = window.inner_size();

        let center_position = PhysicalPosition::new(
            window_size.width as f64 / 2.0,
            window_size.height as f64 / 2.0);

        
        window.set_cursor_position(center_position).unwrap();
        window.set_cursor_visible(false);

        inputs.cursor_position = center_position;


        if inputs.is_keyboard_pressed(VirtualKeyCode::Escape) {
            self.is_exit = true;
        }


        let camera_movement = self.camera_component.clone().unwrap();
        let mut camera_movement = camera_movement.borrow_mut();
        let camera_movement = camera_movement.as_any();
        let camera_movement: &mut CameraMovement = camera_movement.downcast_mut().unwrap();
        
        camera_movement.set_screen_size(window_size);
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