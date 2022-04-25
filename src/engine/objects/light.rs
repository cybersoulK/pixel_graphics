use super::{Transform, ComponentVec};


pub struct Light {
    pub transform: Transform,
    pub components: ComponentVec,
}

impl Light {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            components: ComponentVec::new(),
        }
    }
}
