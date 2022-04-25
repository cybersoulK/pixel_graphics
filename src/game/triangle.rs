use std::rc::Rc;

use glam::{Vec3, Vec2};

use super::Mesh;


pub fn get_triangle() -> Rc<Mesh> {

    let vertices = [
            Vec3::new(0.0,0.0,0.0),
            Vec3::new(0.0,1.0,0.0),  
            Vec3::new(1.0,1.0,0.0),
        ];
        
    let uv_mappings = [
        Vec2::new(0.33,1.0),
        Vec2::new(0.66,1.0),
        Vec2::new(0.33,0.75),
    ];

    let norms = [
        Vec3::new(0.0,0.0,-1.0),
        Vec3::new(0.0,-1.0,0.0),
        Vec3::new(1.0,0.0, 0.0),
    ];

    let indexes = [
        [0, 0, 0], [1, 1, 1], [2, 2, 2],
    ];
    

    let mesh = Mesh::new(
        vertices.to_vec(), 
        uv_mappings.to_vec(), 
        norms.to_vec(), 
        indexes.to_vec(), 
        0);

    mesh
}