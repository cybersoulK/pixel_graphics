use std::any::Any;

use super::{Component, ComponentPipe, Transform};


pub struct TestComponent {

}

impl TestComponent {
}

impl Component for TestComponent {

    fn update(&mut self, transform: &mut Transform, _params: &ComponentPipe) {
        transform.scale /= 1.001;
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}