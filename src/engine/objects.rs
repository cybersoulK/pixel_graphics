pub use transform::Transform;
pub use components::{Component, ComponentVec, ComponentPipe};
pub use camera::Camera;
pub use drawable_object::DrawableObject;
pub use light::Light;

mod settings;
mod transform;
mod world_orientation;
mod components;
mod camera;
mod drawable_object;
mod light;


pub trait Object {
    fn transform(&mut self) -> &mut Transform;
    fn components(&mut self) -> &mut ComponentVec;

    fn get_update_bundle(&mut self) -> (&mut Transform, &ComponentVec);
}