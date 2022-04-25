use super::{Component, Transform, Engine};


pub struct TestComponent {

}

impl Component for TestComponent {

    fn update(&mut self, _transform: Transform, _engine: &Engine) -> Transform {
        
        transform.scale /= 1.001;
        transform
    }
}