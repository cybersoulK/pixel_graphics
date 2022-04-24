use std::rc::Rc;

use super::{Object, Transform};
use super::super::Model;


#[derive(Clone)]
pub struct DrawableObject {
    pub transform: Transform,

    pub model: Rc<Model>,
}

impl DrawableObject {
    pub fn new(transform: Transform, model: Rc<Model>) -> Self {
        Self { transform, model }
    }
}

impl Object for DrawableObject {
}
