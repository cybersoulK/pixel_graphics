use std::ops::{Sub, Mul, AddAssign};
use std::f32::consts::PI;

use glam::{Vec4, Vec3, Vec2, Vec3Swizzles};

use super::RenderingSettings;


pub enum FrontFace {
    ClockWise,
    CounterClockWise,
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


    let ((min_x, min_y), (max_x, max_y)) = clip_triangle(vertices_2d, buffer_size);

    for y in min_y..max_y {  
        for x in min_x..max_x {

            let point = &Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

            if is_inside_triangle(vertices_2d, point, &settings.front_face) {

                let weights = &calc_barycentric(vertices_2d, point);
                let color = mul_barycentric(weights, colors);

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