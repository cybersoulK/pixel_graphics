use std::ops::{Sub, Mul, AddAssign};
use std::f32::consts::PI;

use glam::{Vec2};


pub enum FrontFace {
    ClockWise,
    CounterClockWise,
}


pub fn is_inside_triangle(vertices_2d: [Vec2; 3], point: Vec2, front_face: FrontFace, is_y_inverted: bool) -> bool {

    fn create_vector(vertex_start: Vec2, vertex_end: Vec2) -> Vec2 {
        vertex_end.clone().sub(vertex_start.clone())
    }

    fn get_angle(vec: Vec2) -> f32 {
        
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


    for index in 0..vertices_2d.len() {

        let mut vector_side = create_vector(vertices_2d[index], 
            if index + 1 < 3 { vertices_2d[index + 1] } else { vertices_2d[0] });

        let mut vector_point = create_vector(vertices_2d[index], point);

        if is_y_inverted == true {
            vector_side.y *= -1.0;
            vector_point.y *= -1.0;
        }
            
        let angle_side = get_angle(vector_side);
        let angle_point = get_angle(vector_point);

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


pub fn clip_triangle(vertices_2d: [Vec2; 3], buffer_size: Vec2) -> ((usize, usize), (usize, usize)) {

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


pub fn calc_barycentric(vertices: [Vec2; 3], point: Vec2) -> [f32; 3] {

    let w_div: f32 = (vertices[1].y - vertices[2].y) * (vertices[0].x - vertices[2].x) + (vertices[2].x - vertices[1].x) * (vertices[0].y - vertices[2].y);

    let w1: f32 =  ((vertices[1].y - vertices[2].y) * (point.x - vertices[2].x) + (vertices[2].x - vertices[1].x) * (point.y - vertices[2].y)) / w_div;
    let w2: f32 = ((vertices[2].y - vertices[0].y) * (point.x - vertices[2].x) + (vertices[0].x - vertices[2].x) * (point.y - vertices[2].y)) / w_div;
    let w3: f32 = 1.0 - w1 - w2;

    [w1, w2, w3]
}


pub fn mul_barycentric<V: Mul<f32, Output = V> + AddAssign + Clone + Default>(weights: [f32; 3], vecs: [V; 3]) -> V {

    let mut new_vector = V::default();

    for i in 0..3 {

        let product = vecs[i].clone().mul(weights[i]);
        new_vector.add_assign(product);
    }

    new_vector
}