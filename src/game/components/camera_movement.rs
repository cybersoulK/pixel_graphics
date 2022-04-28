use std::{any::Any, f32::consts::PI};

use winit::dpi::PhysicalSize;

use pixel_graphics::{Component, ComponentPipe, Transform};


const LIMIT_Y_DEGREES: f32 = 150.0 / 2.0 / 180.0 * PI;
const SENSITIVITY: f32 = 0.001;
const SPEED: f32 = 2.0;

pub struct CameraMovement {
    screen_size: PhysicalSize<u32>,
}

impl CameraMovement {
    pub fn new() -> Self {
        Self { 
            screen_size: PhysicalSize { width: 0, height: 0 },
        }
    }

    pub fn set_screen_size(&mut self, screen_size: PhysicalSize<u32>) {
        self.screen_size = screen_size;
    }


    pub fn moving(&mut self, transform: &mut Transform, params: &ComponentPipe) {
        
        let mut vector = glam::Vec3::new(0.0, 0.0, 0.0);


        use winit::event::VirtualKeyCode::*;

        if params.inputs.is_keyboard_pressed(W) { vector.z += 1.0; }
        if params.inputs.is_keyboard_pressed(S) { vector.z -= 1.0; }

        if params.inputs.is_keyboard_pressed(A) { vector.x -= 1.0; }
        if params.inputs.is_keyboard_pressed(D) { vector.x += 1.0; }

        if params.inputs.is_keyboard_pressed(Space) { vector.y += 1.0; }
        if params.inputs.is_keyboard_pressed(C) { vector.y -= 1.0; }

        if vector == glam::vec3(0.0, 0.0, 0.0) { return; }

        let vector = glam::Mat4::from_axis_angle(glam::vec3(0.0, 1.0, 0.0), transform.rotation.y).transform_point3(vector);


        let delta_time = params.timer.delta_time.as_secs_f32();
        transform.translation += vector.normalize() * SPEED * delta_time;
    }

    pub fn looking(&mut self, transform: &mut Transform, params: &ComponentPipe) {

        let mut x = transform.rotation.y;
        let mut y = transform.rotation.x;

        let cursor_x = params.inputs.cursor_position.x as f32 - self.screen_size.width as f32 / 2.0;
        let cursor_y = params.inputs.cursor_position.y as f32 - self.screen_size.height as f32 / 2.0;

        x += cursor_x * SENSITIVITY;
        y += cursor_y * SENSITIVITY;

        if y > LIMIT_Y_DEGREES { y = LIMIT_Y_DEGREES; }
        else if y < -LIMIT_Y_DEGREES { y = -LIMIT_Y_DEGREES; }

        transform.rotation.y = x;
        transform.rotation.x = y;
    }
 }



impl Component for CameraMovement {

    fn update(&mut self, transform: &mut Transform, params: &ComponentPipe) {

        self.moving(transform, params);
        self.looking(transform, params);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}


pub fn build () -> CameraMovement {
    CameraMovement::new()
}