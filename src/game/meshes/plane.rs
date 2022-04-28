use std::rc::Rc;

use glam::{Vec3, Vec2};

use pixel_graphics::Mesh;


pub fn build((x_size, y_size): (usize, usize)) -> Rc<Mesh> {
        
    let uv_mappings = [
        Vec2::new(0.0,0.0),
    ];

    let norms = [
        Vec3::new(0.0,1.0, 0.0),
    ];

    let mut vertices = Vec::new();
    let mut indexes = Vec::new();


    let cell_x_size: f32 = 1.0 / x_size as f32;
    let cell_y_size: f32 = 1.0 / y_size as f32;


    for y in 0..y_size + 1 {
        for x in 0..x_size + 1 {
            
            let cell_pos_x = cell_x_size * x as f32;
            let cell_pos_y = cell_y_size * y as f32;

            vertices.push(Vec3::new(cell_pos_x, 0.0, cell_pos_y));
        }
    }

    for y in 0..y_size {
        for x in 0..x_size {

            [(0, 0), (1, 1), (0, 1),
            (0, 0), (1, 0), (1, 1)].map(|(x_offset, y_offset)| {

                let index = (x + x_offset) + (y + y_offset) * (x_size + 1);

                indexes.push([index, 0, 0]);
            });
        }
    }
    

    let mesh = Mesh::new(
        vertices.to_vec(), 
        uv_mappings.to_vec(), 
        norms.to_vec(), 
        indexes.to_vec(), 
        0);

    mesh
}