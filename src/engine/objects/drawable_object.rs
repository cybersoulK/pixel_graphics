use std::rc::Rc;

use super::{Transform, ComponentVec};
use super::super::Model;


#[derive(Clone)]
pub struct DrawableObject {
    pub transform: Transform,
    pub components: ComponentVec,

    pub model: Rc<Model>,
}

impl DrawableObject {
    pub fn new(transform: Transform, model: Rc<Model>) -> Self {
        Self {
            transform,
            components: ComponentVec::new(),
            
            model,
        }
    }
}