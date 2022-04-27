use std::any::Any;

use super::{Component, ComponentPipe, Transform};

pub struct TestComponent {

}

impl Component for TestComponent {

    fn update(&mut self, transform: &mut Transform, params: &ComponentPipe) {

        let delta_time =  params.timer.delta_time.as_secs_f32();
        
        transform.rotation.y += 0.5 * delta_time;
        transform.rotation.x = 0.5;
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}