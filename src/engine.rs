use std::{time::{Instant, Duration}};

pub use events::*;
pub use graphics::*;
pub use inputs::*;
pub use assets::*;
pub use objects::*;

mod events;
mod graphics;
mod inputs;
mod objects;
mod assets;



pub struct Engine {
    pub assets: Assets,

    drawables: Vec<DrawableObject>,
    lights: Vec<Light>,
    camera: Camera,

    timer: EngineTimer,
    inputs: InputsManager,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            assets: Assets::new(),

            drawables: Vec::new(),
            lights: Vec::new(),
            camera: Default::default(),

            timer: EngineTimer::new(),
            inputs: InputsManager::new(),
        }
    }

    pub fn update(&mut self){

        self.timer.update();


        let update_pipe = ComponentPipe {
            timer: self.timer,
            inputs: self.inputs.clone(),
        };

        Self::update_object(&mut self.camera, &update_pipe);
        
        for obj in self.drawables.iter_mut() {
            Self::update_object(obj, &update_pipe); }

        for obj in self.lights.iter_mut() {
            Self::update_object(obj, &update_pipe); }
    }

    fn update_object<O : Object>(obj: &mut O, pipe: &ComponentPipe){

        let (transform, components) = obj.get_update_bundle();
 
        for c in components.iter() {
            c.borrow_mut().update(transform, &pipe);
        }
        transform.update_matrix();
    }


    pub fn set_inputs(&mut self, inputs: InputsManager) {
        self.inputs = inputs;
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



#[derive(Clone, Copy)]
pub struct EngineTimer {
    time_program_start: Instant,
    time_now: Instant,
    
    pub delta_time: Duration,
    pub elapsed_time: Duration,
}

impl EngineTimer {
    fn new() -> Self {
        Self {
            time_program_start: Instant::now(),
            time_now: Instant::now(),

            delta_time: Duration::from_secs_f32(0.01),
            elapsed_time: Duration::new(0, 0),
        }
    }

    fn update(&mut self) {

        let time_now = Instant::now();

        self.delta_time = time_now - self.time_now;
        self.time_now = time_now;

        self.elapsed_time = self.time_now - self.time_program_start;
    }

}