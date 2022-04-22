use std::{collections::HashMap};

use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop},
};


use pixel_renderer::PixelRenderer;

pub use assets::*;
pub use objects::*;

mod pixel_renderer;
mod objects;
mod assets;


pub struct Engine {
    pub assets: Assets,
    objects: Vec<Box<dyn objects::Object>>,
}

impl Engine {
    fn new() -> Self {
        Self {
            assets: Assets::new(),
            objects: Vec::new(),
        }
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
                renderer.render(); //&engine.objects
            },
            _ => ()
        }


    });
}