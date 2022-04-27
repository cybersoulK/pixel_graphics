use std::collections::HashMap;

use winit::{
    event::{MouseButton, ElementState, VirtualKeyCode},
    dpi::PhysicalPosition
};


#[derive(Clone)]
pub struct InputsManager {
    keyboard_inputs: HashMap<VirtualKeyCode, ()>,
    mouse_inputs: HashMap<MouseButton, ()>,

    pub cursor_position: PhysicalPosition<f64>,
}

impl InputsManager {
    pub fn new() -> Self {
        Self {
            keyboard_inputs: HashMap::new(),
            mouse_inputs: HashMap::new(),

            cursor_position: PhysicalPosition::new(0.0, 0.0),
        }
    }

    pub fn on_keyboard_input(&mut self, keycode: Option<VirtualKeyCode>, state: ElementState) {

        use ElementState::*;

        if let Some(keycode) = keycode {

            match state {
                Pressed => {
                    self.keyboard_inputs.insert(keycode, ());
                },
                Released => {
                    self.keyboard_inputs.remove(&keycode);
                }
            }
        }
        
        
    }
    pub fn is_keyboard_pressed(&self, keycode: VirtualKeyCode) -> bool {
        
        if self.keyboard_inputs.get(&keycode).is_some() {
            true
        }
        else {
            false
        }
    }

    pub fn on_mouse_input(&mut self, button: MouseButton, state: ElementState) {

        use ElementState::*;
        
        match state {
            Pressed => {
                self.mouse_inputs.insert(button, ());
            },
            Released => {
                self.mouse_inputs.remove(&button);
            }
        }
    }
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {

        if self.mouse_inputs.get(&button).is_some() {
            true
        }
        else {
            false
        }
    }

    pub fn on_cursor_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_position = position;
    }
}