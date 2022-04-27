use winit::dpi::{LogicalSize, PhysicalSize};
use winit::window::{WindowBuilder, Window};

use winit::event_loop::{ControlFlow, EventLoop};

use pixels::{SurfaceTexture, PixelsBuilder};
use pixels::wgpu::{PowerPreference, RequestAdapterOptions, Color};

use glam::Vec2;

use super::super::Engine;
use super::render_update;


pub struct WindowPixel {
    pub window: Window,
    context: pixels::Pixels,

    z_buffer: Vec<f32>,

    buffer_size: LogicalSize<u32>,
}

impl WindowPixel {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {

        let window = WindowBuilder::new().build(event_loop).unwrap();
        
        let surface_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(100, 100, &window);
        
        let mut context = PixelsBuilder::new(100, 100, surface_texture)
        .request_adapter_options(RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .enable_vsync(false)
        .build().unwrap();

        context.set_clear_color(Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        });

        
        let mut renderer = Self {
            window,
            context,

            z_buffer: Vec::new(),

            buffer_size: LogicalSize::default(),
        };

        renderer.resize(surface_size);

        renderer
    }

    pub fn render(&mut self, engine: &Engine) {

        let buffer = self.context.get_frame();  
        let z_buffer = self.z_buffer.as_mut_slice(); 
        
        let buffer_size = Vec2::new(self.buffer_size.width as f32, self.buffer_size.height as f32);

        render_update(
            buffer,
            z_buffer,
            buffer_size,
            &engine.camera,
            &engine.drawables,
            &engine.lights,
            engine.timer.elapsed_time);
                    
            self.context.render().unwrap();
            self.window.request_redraw();
    }
    
    
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {

        self.context.resize_surface(new_size.width, new_size.height);


        let smaller = if new_size.width < new_size.height { new_size.width } else { new_size.height };
        
        let new_buffer_width = (super::settings::MIN_BUFFER_SIDE as f32 * new_size.width as f32 / smaller as f32) as u32;
        let new_buffer_height =  (super::settings::MIN_BUFFER_SIDE as f32 * new_size.height as f32 / smaller as f32) as u32;

        self.context.resize_buffer(new_buffer_width, new_buffer_height);

        self.z_buffer.clear();
        self.z_buffer.extend((0 .. new_buffer_width * new_buffer_height).map(|_| f32::MAX));

        self.buffer_size = LogicalSize::new(new_buffer_width, new_buffer_height);
        //BUG(from pixels.rs): buffer only scales by 1x 2x 4x according to the surface area.
    }
}