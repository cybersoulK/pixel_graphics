use std::f32::consts::PI;

use glam::Vec3;

use pixel_graphics::Light;


pub fn spot_lighting(vertex: Vec3, norm: Vec3, light: &Light) -> f32 {
    
    let distance = vertex.distance(light.transform.translation);
    let norm = norm.dot(light.transform.translation - vertex);
    
    const INTENSITY: f32 = 4.0;
    const INTENSITY_RANGE: f32 = 0.3;
    const INTENSITY_CURVE: f32 = 0.8;

    let light_intensity = 
        (4.0 * PI * INTENSITY_RANGE.powf(INTENSITY_CURVE))
        / (4.0 * PI * distance.powf(INTENSITY_CURVE)) 
        * INTENSITY * norm;


    light_intensity
}

pub fn directional_light(norm: Vec3, light_direction: Vec3) -> f32 {
    
    let norm = norm.dot(light_direction * -1.0);

    const INTENSITY: f32 = 0.7;

    let light_intensity = INTENSITY * norm;

        
    light_intensity
}