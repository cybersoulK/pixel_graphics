use std::{rc::Rc, cell::RefCell, vec::IntoIter, slice::Iter};

use super::{Transform, super::Engine};


pub trait Component {
    //fn init(&mut self, _transform: Transform) -> Transform; // _engine: &Engine
    fn update(&mut self, _transform: Transform, _engine: &Engine) -> Transform; 
}

#[derive(Clone)]
pub struct ComponentVec {
    vec: Vec<Rc<dyn Component>>
}

impl ComponentVec {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
        }
    }

    pub fn add<C: Component + 'static>(&mut self, component: C){

        let d = Rc::new(component);
        self.vec.push(d);
    }

    pub fn iter(&self) -> Iter<Rc<dyn Component>> {
        self.vec.iter()
    }
}