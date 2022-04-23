use std::rc::Rc;

use super::{Object, Transform};


pub struct Light {
    pub transform: Transform,
}

impl Light {
    pub fn new(transform: Transform) -> Self {
        Self { transform }
    }
}

impl Object for Light {
}
