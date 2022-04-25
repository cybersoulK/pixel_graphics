use std::{rc::Rc};
use std::time::Instant;

use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop},
};

pub use graphics::*;
pub use assets::*;
pub use objects::*;

mod graphics;
mod objects;
mod assets;


pub struct Engine {
    pub assets: Assets,

    drawables: Vec<DrawableObject>,
    lights: Vec<Light>,

    camera: Camera,

    program_start: Instant,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            assets: Assets::new(),

            drawables: Vec::new(),
            lights: Vec::new(),

            camera: Default::default(),

            program_start: Instant::now(),
        }
    }

    pub fn update(&mut self){

        self.camera.transform.update_matrix();
        for c in self.camera.components.iter() {
            self.camera.transform = c.borrow_mut().update(self.camera.transform);
        }


        for obj in self.drawables.iter_mut() {
            obj.transform.update_matrix();
        }

        for obj in self.drawables.iter_mut() {
            for c in obj.components.iter() {
                obj.transform = c.borrow_mut().update(obj.transform);
            }
        }


        for obj in self.lights.iter_mut() {
            obj.transform.update_matrix();
        }

        for obj in self.lights.iter_mut() {
            for c in obj.components.iter() {
                obj.transform = c.borrow_mut().update(obj.transform);
            }
        }
    }


    pub fn set_camera(&mut self, transform: Transform, near: f32, fov_y_radians: f32) {

        self.camera = Camera::new(transform, near, fov_y_radians);
    }

    pub fn add_drawable(&mut self, drawable: DrawableObject) {

        self.drawables.push(drawable);
    }

    pub fn add_light(&mut self, light: Light) {

        self.lights.push(light);
    }
}

pub trait GameLoop {
    fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine);

    //fn on_key_down(keycode) {}
    //fn on_key_up(keycode) {}
}


pub fn init<T>(mut game_loop: T)
    where T: GameLoop + 'static {

    let event_loop = EventLoop::new();

    let mut engine = Engine::new();
    let mut window = WindowPixel::new(&event_loop);


    game_loop.init(&mut engine);


    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Poll;
        

        match event {
            Event::WindowEvent { event, .. } => window.window_event(event, control_flow),

            Event::MainEventsCleared => {

                game_loop.update(&mut engine);
                engine.update();


                let (buffer, z_buffer, buffer_size) = window.get_buffer();

                graphics::render_update(
                    buffer,
                    z_buffer,
                    buffer_size,
                    &engine.camera,
                    &engine.drawables,
                    &engine.lights,
                    engine.program_start.elapsed());
                    
                window.refresh();
            },
            _ => ()
        }


    });
}