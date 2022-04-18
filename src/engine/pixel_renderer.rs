use winit::dpi::{LogicalSize, PhysicalSize};
use winit::window::{WindowBuilder, Window};

use winit::{
    event::{WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use pixels::{SurfaceTexture, PixelsBuilder};
use pixels::wgpu::{PowerPreference, RequestAdapterOptions, Color};

use quad_rand::RandomRange;



const MIN_BUFFER_SIDE: u32 = 800;

pub struct PixelRenderer {
    window: Window,
    context: pixels::Pixels,

    buffer_size: LogicalSize<u32>,
}


impl PixelRenderer {
    pub fn new<T>(event_loop: &EventLoop<T>) -> PixelRenderer {

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
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        });

        
        let mut renderer = PixelRenderer {
            window,
            context,

            buffer_size: LogicalSize::default(),
        };

        renderer.resize(surface_size);

        renderer
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {

        self.context.resize_surface(new_size.width, new_size.height);


        let smaller = if new_size.width < new_size.height { new_size.width } else { new_size.height };
        
        let new_buffer_width = (MIN_BUFFER_SIDE as f32 * new_size.width as f32 / smaller as f32) as u32;
        let new_buffer_height =  (MIN_BUFFER_SIDE as f32 * new_size.height as f32 / smaller as f32) as u32;

        self.context.resize_buffer(new_buffer_width, new_buffer_height);

        self.buffer_size = LogicalSize::new(new_buffer_width, new_buffer_height);
        // BUG(from pixels.rs): buffer only scales by 1x 2x 4x according to the surface area.
    }

    pub fn window_event(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {

        match event {
    
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            WindowEvent::Resized(new_size) => {
                self.resize(new_size);
            },
            WindowEvent::ScaleFactorChanged  { new_inner_size, ..} => {
                self.resize(*new_inner_size);
            },
            _ => ()
        }
    }

    pub fn render(&mut self){

        let buffer = self.context.get_frame();
                    
        for i in 0..(self.buffer_size.width * self.buffer_size.height) as usize {
            buffer[i * 4 + 0] = RandomRange::gen_range(0, 50);
            buffer[i * 4 + 1] = RandomRange::gen_range(50, 130);
            buffer[i * 4 + 2] = RandomRange::gen_range(130, 255);
            buffer[i * 4 + 3] = 255;
        } 
    
        self.context.render().unwrap();

        self.window.request_redraw();
    }
}