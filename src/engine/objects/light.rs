use super::{Object, Transform, ComponentVec};


#[derive(Clone)]
pub struct Light {
    pub transform: Transform,
    pub components: ComponentVec,

    //TODO pub intensity; 
}

impl Light {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            components: ComponentVec::new(),
        }
    }
}


impl Object for Light {
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