use std::any::Any;

use pixel_graphics::{Component, ComponentPipe, Transform};


pub struct CustomComponent {
}

impl Component for CustomComponent {

    fn update(&mut self, transform: &mut Transform, _params: &ComponentPipe) {
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}


pub fn build () -> CustomComponent {
    CustomComponent {}
}