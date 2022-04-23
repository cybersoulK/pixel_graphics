use std::rc::Rc;
use super::Shader;


pub struct Shader1 {

}

impl Shader for Shader1 {
    fn new() -> Rc<Shader1> {
        Rc::new(Self {
            
        })
    }
}