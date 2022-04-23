use glam::Vec3;
use glam::EulerRot;
use glam::Mat4;

pub use camera::Camera;
pub use drawable_object::DrawableObject;
pub use light::Light;

mod camera;
mod drawable_object;
mod light;


#[derive(Default)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: EulerRot,
    pub scale: Vec3,

    pub matrix: Mat4,
}

pub trait Object {}



