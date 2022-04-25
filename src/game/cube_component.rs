use super::{Component, Transform, Engine};

pub struct TestComponent {

}

impl Component for TestComponent {

    fn update(&mut self, _transform: Transform, _engine: &Engine) -> Transform { //engine: &Engine
        
        transform.rotation.y += 0.005;
        transform.rotation.x += 0.003;
        transform
    }
}