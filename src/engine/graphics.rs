pub use window::WindowPixel;
pub use shader_pipe::{CorePipe, ParamsPipe, DynamicPipe};
pub use render::render_update;

mod settings;
mod window;
mod render;
mod triangles;
mod shader_pipe;