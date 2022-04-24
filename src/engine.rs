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

    objects: Vec<Rc<dyn Object>>,
    drawables: Vec<Rc<DrawableObject>>,
    lights: Vec<Rc<Light>>,

    camera: Rc<Camera>,

    program_start: Instant,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            assets: Assets::new(),

            objects: Vec::new(),
            drawables: Vec::new(),
            lights: Vec::new(),

            camera: Default::default(),

            program_start: Instant::now(),
        }
    }

    pub fn set_camera(&mut self, transform: Transform, near: f32, far: f32, fov: f32) {

        let camera = Rc::new(Camera::new(transform, near, far, fov));
        let camera_dyn: Rc<dyn Object> = Rc::<Camera>::clone(&camera);

        self.camera = Rc::clone(&camera);
        self.objects.push(Rc::clone(&camera_dyn));
    }

    pub fn add_object(&mut self, object: Rc<dyn Object>) {

        self.objects.push(object);
    } 

    pub fn add_drawable(&mut self, drawable: Rc<DrawableObject>) {

        self.objects.push(Rc::<DrawableObject>::clone(&drawable));
        self.drawables.push(Rc::clone(&drawable));
    }

    pub fn add_light(&mut self, light: Rc<Light>) {

        self.objects.push(Rc::<Light>::clone(&light));
        self.lights.push(Rc::clone(&light));
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


                let (buffer, buffer_size) = window.get_buffer();

                graphics::render_update(
                    buffer,
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