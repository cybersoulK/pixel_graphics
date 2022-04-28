use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

use pixel_graphics::Object;

use pixel_graphics::{Transform, Camera};

use crate::game::components::camera_movement::{build as build_component, CameraMovement};



pub fn build() -> (Camera, Rc<RefCell<CameraMovement>>) {
    
    let camera_transform = Transform::default();
    let mut camera = Camera::new(camera_transform, 0.01, (70.0 / 90.0) * (PI / 2.0));
    
    let camera_component = build_component();
    let camera_component = camera.components().add(camera_component);

    (camera, camera_component)
}