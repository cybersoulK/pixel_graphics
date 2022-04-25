use super::{Component, Transform, Engine};

pub struct TestComponent {

}

impl Component for TestComponent {

    fn update(&mut self, mut transform: Transform) -> Transform { //engine: &Engine
        
        transform.rotation.y += 0.001;
        transform.rotation.x += 0.0005;
        transform
    }
}