use std::{rc::Rc, cell::RefCell, slice::Iter, any::Any};

use super::super::{
    Transform,
    EngineTimer,
    InputsManager,
};


pub trait Component {
    fn init(&mut self, _transform: &mut Transform) {}
    fn update(&mut self, _transform: &mut Transform, _params: &ComponentPipe) {}

    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct ComponentPipe {
    pub timer: EngineTimer,
    pub inputs: InputsManager,
}


#[derive(Clone)]
pub struct ComponentVec {
    vec: Vec<Rc<RefCell<dyn Component>>>
}

impl ComponentVec {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
        }
    }

    pub fn add<C: Component + 'static>(&mut self, component: C) -> Rc<RefCell<C>> {

        let c = Rc::new(RefCell::new(component));
        self.vec.push(c.clone());

        c
    }

    pub fn iter(&self) -> Iter<Rc<RefCell<dyn Component>>> {
        self.vec.iter()
    }
}