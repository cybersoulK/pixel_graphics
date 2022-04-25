use super::{Component, Transform, Engine};


pub struct CameraMovement {

}

impl Component for CameraMovement {

    fn update(&mut self, _transform: Transform, _engine: &Engine) -> Transform {
        //transform.translation.x -= 0.001;
        //transform.translation.y -= 0.001;

        transform
    }
}