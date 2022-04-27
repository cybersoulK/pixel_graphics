use std::rc::Rc;

use super::{Object, Transform, ComponentVec};
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


impl Object for DrawableObject {
    fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn components(&mut self) -> &mut ComponentVec {
        &mut self.components
    }

    fn get_update_bundle(&mut self) -> (&mut Transform, &ComponentVec) {
        (&mut self.transform, &self.components)
    }
}