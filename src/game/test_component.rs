use super::{Component, Transform, Engine};


pub struct TestComponent {

}

impl Component for TestComponent {

    fn update(&mut self, transform: &Transform, engine: &Engine) {
        println!("sd");
    }
}