use std::rc::Rc;

use glam::{Vec3, Vec2};

use super::Mesh;


pub fn get_cube() -> Rc<Mesh> {

    let vertices = [
            Vec3::new(0.0,0.0,0.0), //0
            Vec3::new(1.0,0.0,0.0), //1  
            Vec3::new(0.0,1.0,0.0), //2 
            Vec3::new(1.0,1.0,0.0), //3

            Vec3::new(0.0,0.0,1.0), //4
            Vec3::new(1.0,0.0,1.0), //5
            Vec3::new(0.0,1.0,1.0), //6
            Vec3::new(1.0,1.0,1.0), //7
        ];
        
    let uv_mappings = [
        // bottom
        Vec2::new(0.33,1.0),  //0
        Vec2::new(0.66,1.0),  //1
        Vec2::new(0.33,0.75), //2
        Vec2::new(0.66,0.75), //3
        // left - back - right
        Vec2::new(0.0,0.5),   //4
        Vec2::new(0.33,0.5),  //5
        Vec2::new(0.66,0.5),  //6
        Vec2::new(1.0,0.5),   //7
        // left - top - right 
        Vec2::new(0.0,0.25),  //8
        Vec2::new(0.33,0.25), //9
        Vec2::new(0.66,0.25), //10
        Vec2::new(1.0,0.25),  //11
        // front
        Vec2::new(0.33,0.0),  //12
        Vec2::new(0.66,0.0),  //13
    ];

    let norms = [
        Vec3::new(0.0,0.0,-1.0), //bottom
        Vec3::new(0.0,-1.0,0.0), //front
        Vec3::new(1.0,0.0, 0.0), //right
        Vec3::new(-1.0,0.0,0.0), //left
        Vec3::new(0.0,1.0,0.0), //back
        Vec3::new(0.0,0.0,1.0), //top
    ];

    let indexes = [
        // bottom
        [0, 9, 0], [2, 5, 0], [3, 6, 0],   [0, 9, 0], [3, 6, 0], [1, 10, 0],
        // front
        [0, 9, 1], [5, 13, 1], [4, 12, 1],   [0, 9, 1], [1, 10, 1], [5, 13, 1],
        // right
        [2, 10, 2], [7, 7, 2], [5, 11, 2],   [2, 10, 2], [3, 6, 2], [7, 7, 2],
        // left
        [2, 5, 3], [4, 8, 3], [6, 4, 3],   [2, 5, 3], [0, 9, 3], [4, 8, 3],
        // back
        [3 , 6, 4], [6, 2, 4], [7, 3, 4],   [3, 6, 4], [2, 5, 4], [6, 2, 4],
        // top
        [4, 0, 5], [7, 3, 5], [6, 2, 5],   [4, 0, 5], [5, 1, 5], [7, 3, 5]];
        
    /*
    023 031 //bottom
    054 015   //front
    275 237  //right
    246 204 //left
    367 326 // back
    476 457 //top

    9/5/6 9/6/10
    9/13/12 9/10/13
    10/7/11 10/6/7
    5/8/4 5/9/8
    6/2/3 6/5/2
    0/3/2 0/1/3
    */

    let mesh = Mesh::new(
        vertices.to_vec(), 
        uv_mappings.to_vec(), 
        norms.to_vec(), 
        indexes.to_vec(), 
        0);

    mesh
}