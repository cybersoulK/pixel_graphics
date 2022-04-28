use std::any::Any;

use pixel_graphics::{Component, ComponentPipe, Transform};


pub struct CustomComponent {

}

impl Component for CustomComponent {

    fn update(&mut self, transform: &mut Transform, params: &ComponentPipe) {

        let delta_time =  params.timer.delta_time.as_secs_f32();
        
        transform.rotation.y += 0.5 * delta_time;
        transform.rotation.x = 0.5;
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}


pub fn build () -> CustomComponent {
    CustomComponent {}
}