use super::{Component, Transform, Engine};


pub struct TestComponent {

}

impl Component for TestComponent {

    fn update(&mut self, mut transform: Transform) -> Transform { //engine: &Engine
        
        transform.scale /= 1.001;
        transform
    }
}