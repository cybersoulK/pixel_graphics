use std::rc::Rc;

use super::{Object, Transform};
use crate::engine::assets::Model;

pub struct DrawableObject {
    transform: Transform,

    model: Rc<Model>,
}

impl DrawableObject {
    pub fn new(transform: Transform, model: Rc<Model>) -> Self {
        Self { transform, model }
    }
}

impl Object for DrawableObject {
}