use std::f32::consts::PI;
use std::ops::{Sub, Mul, AddAssign};

use winit::dpi::{LogicalSize, PhysicalSize};
use winit::window::{WindowBuilder, Window};

use winit::{
    event::{WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use pixels::{SurfaceTexture, PixelsBuilder};
use pixels::wgpu::{PowerPreference, RequestAdapterOptions, Color};

use glam::{Vec4, Vec3, Vec2, IVec2, Vec2Swizzles, Vec3Swizzles, Vec4Swizzles};



const MIN_BUFFER_SIDE: u32 = 2000;


pub struct PixelRenderer {
    window: Window,
    context: pixels::Pixels,

    buffer_size: LogicalSize<u32>,
    time: std::time::SystemTime,
}


impl PixelRenderer {             
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

            buffer_size: LogicalSize::default(),
            time: std::time::SystemTime::now(),
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
        //BUG(from pixels.rs): buffer only scales by 1x 2x 4x according to the surface area.
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
        buffer.fill(0);
                    

        let mut vertices = [
            Vec3::new(0.5, 0.7, 0.0),
            Vec3::new(0.3, 0.4, 0.0),
            Vec3::new(0.7, 0.4, 0.0),
        ];

        let mut vertices2 = [
            Vec3::new(0.3, 0.4, 0.0),
            Vec3::new(0.5, 0.1, 0.0),
            Vec3::new(0.7, 0.4, 0.0),
        ];

        for vec in vertices.iter_mut() {
            vec.x *= self.buffer_size.width as f32;
            vec.y *= self.buffer_size.height as f32;
        }

        for vec in vertices2.iter_mut() {
            vec.x *= self.buffer_size.width as f32;
            vec.y *= self.buffer_size.height as f32;
        }

        //test only
        for vec in vertices.iter_mut() {
            vec.x += (self.time.elapsed().unwrap().as_secs_f32() * PI * 0.2).cos() * 200.0;
            vec.y += (self.time.elapsed().unwrap().as_secs_f32() * PI * 0.2).sin() * 200.0;
        }

        //test only
        for vec in vertices2.iter_mut() {
            vec.x += (self.time.elapsed().unwrap().as_secs_f32() * PI * 0.2).cos() * 200.0;
            vec.y += (self.time.elapsed().unwrap().as_secs_f32() * PI * 0.2).sin() * 200.0;
        }

        let colors = [
            Vec4::new(1.0, 0.2, 0.0, 1.0),
            Vec4::new(0.2, 0.0, 0.8, 1.0),
            Vec4::new(0.0, 1.0, 0.2, 1.0),
        ];

        let colors2 = [
            Vec4::new(0.2, 0.0, 0.8, 1.0),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            Vec4::new(0.0, 1.0, 0.2, 1.0),
        ];


        let settings = RenderingSettings { front_face: FrontFace::CounterClockWise };

        
        let buffer_size = Vec2::new(self.buffer_size.width as f32, self.buffer_size.height as f32);
        
        Self::draw_triangle(buffer, &buffer_size, &vertices, &colors, &settings);
        Self::draw_triangle(buffer, &buffer_size, &vertices2, &colors2, &settings);
    
        self.context.render().unwrap();
        self.window.request_redraw();
    }



    fn is_inside_triangle(vertices_2d: &[Vec2], point: &Vec2, front_face: &FrontFace) -> bool {

        fn create_vector(vertex_1: &Vec2, vertex_2: &Vec2) -> Vec2 {
            vertex_2.clone().sub(vertex_1.clone())
        }

        fn get_angle(vec: &Vec2) -> f32 {
            
            let radius = (vec.x.powf(2.0) + vec.y.powf(2.0)).sqrt();
            let mut angle;

            if vec.x > 0.0 && vec.y > 0.0 {

                let sin = vec.y / radius;
                angle = sin.asin();
            }
            else {
                
                let cos = vec.x / radius;
                angle = cos.acos();

                if vec.y < 0.0 { angle = PI * 2.0 - angle}
            }

            angle
        }


        for index in 0 .. vertices_2d.len() {

            let vector_side = create_vector(&vertices_2d[index], 
                if index == 2 { &vertices_2d[0] } else { &vertices_2d[index+1] });

            let vector_point = create_vector(&vertices_2d[index], &point);


            let angle_side = get_angle(&vector_side);
            let angle_point = get_angle(&vector_point);

            let mut angle_dif = angle_point - angle_side;
            if angle_dif < 0.0 { angle_dif += PI * 2.0; }


            if let FrontFace::ClockWise = front_face  { angle_dif = PI * 2.0 - angle_dif; }
            else {}
   
            
            if angle_dif > PI {
                return false
            }
        }
        
        true
    }
    

    fn draw_triangle(buffer: &mut[u8], buffer_size: &Vec2, vertices: &[Vec3; 3], colors: &[Vec4; 3], settings: &RenderingSettings) {

        let vertices_2d = &[
            vertices[0].clone().xy(),
            vertices[1].clone().xy(),
            vertices[2].clone().xy(),
        ];

        let colors = &[
            colors[0].mul(255.0),
            colors[1].mul(255.0),
            colors[2].mul(255.0),
        ];


        let ((min_x, min_y), (max_x, max_y)) = Self::clip_triangle(vertices_2d, buffer_size);

        for y in min_y..max_y {  
            for x in min_x..max_x {
  
                let point = &Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
   
                if Self::is_inside_triangle(vertices_2d, point, &settings.front_face) {
   
                    let weights = &Self::calc_barycentric(vertices_2d, point);
                    let color = Self::mul_barycentric(weights, colors);

                    let pixel_index = (y * buffer_size.x as usize + x) * 4; 

                    buffer[pixel_index + 0] = color.x as u8;
                    buffer[pixel_index + 1] = color.y as u8;
                    buffer[pixel_index + 2] = color.z as u8;
                    buffer[pixel_index + 3] = color.w as u8;
                }
            }
        }
    }


    fn clip_triangle(vertices_2d: &[Vec2], buffer_size: &Vec2) -> ((usize, usize), (usize, usize)) {

        let mut min = vertices_2d[0].clone();
        let mut max = vertices_2d[0].clone();

        for v in vertices_2d.iter().skip(1) {
            min = min.min(v.clone());
            max = max.max(v.clone());
        }


        let min = min.max(Vec2::new(0.0, 0.0).clone()).round();
        let max = max.min(buffer_size.clone()).round();

        return ((min.x as usize, min.y as usize), (max.x as usize, max.y as usize));
    }


    fn calc_barycentric(vertices: &[Vec2], point: &Vec2) -> [f32; 3] {

        let w_div: f32 = (vertices[1].y - vertices[2].y) * (vertices[0].x - vertices[2].x) + (vertices[2].x - vertices[1].x) * (vertices[0].y - vertices[2].y);

        let w1: f32 =  ((vertices[1].y - vertices[2].y) * (point.x - vertices[2].x) + (vertices[2].x - vertices[1].x) * (point.y - vertices[2].y)) / w_div;
        let w2: f32 = ((vertices[2].y - vertices[0].y) * (point.x - vertices[2].x) + (vertices[0].x - vertices[2].x) * (point.y - vertices[2].y)) / w_div;
        let w3: f32 = 1.0 - w1 - w2;

        [w1, w2, w3]
    }


    fn mul_barycentric<V: Mul<f32, Output = V> + AddAssign + Clone + Default>(weights: &[f32], vecs: &[V; 3]) -> V {

        let mut new_vector = V::default();

        for i in 0..3 {

            let product = vecs[i].clone().mul(weights[i]);
            new_vector.add_assign(product);
        }

        new_vector
    } 
}


struct RenderingSettings {
    pub front_face: FrontFace,
}

enum FrontFace {
    ClockWise,
    CounterClockWise,
}