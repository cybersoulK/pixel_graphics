use std::string::Drain;
use std::{collections::HashMap, rc::Rc};
use std::any::Any;

use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop},
};


use renderer::PixelRenderer;

pub use assets::*;
pub use objects::*;

mod renderer;
mod objects;
mod assets;


pub struct Engine {
    pub assets: Assets,

    objects: Vec<Rc<dyn Object>>,
    drawables: Vec<Rc<DrawableObject>>,
    camera: Rc<Camera>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            assets: Assets::new(),
            objects: Vec::new(),
            drawables: Vec::new(),
            camera: Default::default(),
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


        let any: Rc<dyn Any> = Rc::clone(&object);
        let drawable: Option<DrawableObject> = any.downcast_ref();

        if drawable.is_some() { self.drawables.push(drawable); }
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
    let mut renderer = PixelRenderer::new(&event_loop);


    game_loop.init(&mut engine);


    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Poll;
        

        match event {
            Event::WindowEvent { event, .. } => renderer.window_event(event, control_flow),

            Event::MainEventsCleared => {

                game_loop.update(&mut engine);
                renderer.render(&engine.camera, &engine.objects); //&engine.objects
            },
            _ => ()
        }


    });
}