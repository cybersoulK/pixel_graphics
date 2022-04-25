use std::time::{Instant, Duration};

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

    time_program_start: Instant,
    time_now: Instant,
    deltatime: Duration,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            assets: Assets::new(),

            drawables: Vec::new(),
            lights: Vec::new(),

            camera: Default::default(),

            time_program_start: Instant::now(),
            time_now: Instant::now(),
            deltatime: Duration::from_secs_f32(0.01),
        }
    }

    pub fn update(&mut self){

        self.update_deltatime();
        self.update_components();
    }

    fn update_deltatime(&mut self) {

        let time_now = Instant::now();

        self.deltatime = time_now - self.time_now;
        self.time_now = time_now;
    }

    fn update_components(&mut self) {

        let iter = self.camera.components.iter();

        for obj in self.drawables.iter_mut() {
            iter.chain(obj.components.iter());
        }
        for obj in self.lights.iter_mut() {
            iter.chain(obj.components.iter());
        }



        for c in self.camera.components.iter() {
            self.camera.transform = c.borrow_mut().update(self.camera.transform);
        }
        for c in obj.components.iter() {
            obj.transform = c.borrow_mut().update(obj.transform);
        }
        for c in obj.components.iter() {
            obj.transform = c.borrow_mut().update(obj.transform);
        }



        self.camera.transform.update_matrix();

        for obj in self.drawables.iter_mut() {
            obj.transform.update_matrix();
        }
        for obj in self.lights.iter_mut() {
            obj.transform.update_matrix();
        }

        
    }


    pub fn set_camera(&mut self, camera: Camera) {

        self.camera = camera;
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
                    engine.time_program_start.elapsed());
                    
                window.refresh();
            },
            _ => ()
        }


    });
}