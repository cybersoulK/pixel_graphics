use std::rc::Rc;

use glam::{Vec2, Vec3, Vec4, Mat4, Vec3Swizzles};

use crate::Material;

use super::super::{
    {Shader, VertexPipe, FragmentPipe},
    {Camera, DrawableObject, Light},
    {ModelIterator, MeshIterator}};

use super::triangles;



pub fn render_update(
    buffer: &mut [u8],
    buffer_size: Vec2,
    camera: &Rc<Camera>,
    drawables: &Vec<Rc<DrawableObject>>,
    lights: &Vec<Rc<Light>>) {


    buffer.fill(0);


    let vp_matrix = camera.get_vp_matrix();

    for obj in drawables {

        let mvp_matrix = obj.transform.matrix * vp_matrix;
        let mvp_matrix = vp_matrix; //TODO: check for error: should error, moved before

        let model = Rc::clone(&obj.model);


        for (mesh, material) in ModelIterator::new(&model) {
            
            let shader = material.get_shader();
            
            
            for (vertices, uv_mappings, norms) in MeshIterator::new(&mesh) {
                
                let colors = [None; 3];

                let vertex_output = 
                execute_vertex_shader(
                    &shader,
                    vertices, 
                    uv_mappings,
                    norms,
                    colors,
                    mvp_matrix);
                
                execute_fragment_shader(
                    buffer,
                    buffer_size,
                    &shader,
                    vertex_output,
                    Rc::clone(&material),
                    lights,
                );
            }
        }
    }
}

fn execute_vertex_shader(
    shader: &Rc<dyn Shader>,
    vertices: [Vec3; 3], 
    uv_mappings: [Vec2; 3], 
    norms: [Vec3; 3],
    colors: [Option<Vec4>; 3],
    mvp_matrix: Mat4) -> [VertexPipe; 3] {

    
    let vertex_output = [0, 1, 2].map(|i| {

        let input = VertexPipe {
            id: i,

            vertex: vertices[i],
            uv_mapping: uv_mappings[i],
            norm: norms[i],

            color: colors[i],

            mvp_matrix,
        };

        shader.vertex_shader(input)
    });

    vertex_output
}

fn execute_fragment_shader(
    buffer: &mut [u8],
    buffer_size: Vec2,
    shader: &Rc<dyn Shader>,
    vertex_output: [VertexPipe; 3],
    material: Rc<Material>,
    lights: &Vec<Rc<Light>>) {

    
    let vertices = vertex_output.map(|v| v.vertex);
    let uv_mappings = vertex_output.map(|v| v.uv_mapping);
    let norms = vertex_output.map(|v| v.norm);

    let colors = VertexPipe::transform_option_bundle(vertex_output.map(|v| v.color));


    let vertices_2d = &[
        vertices[0].clone().xy(),
        vertices[1].clone().xy(),
        vertices[2].clone().xy(),
    ];

    let ((min_x, min_y), (max_x, max_y)) = triangles::clip_triangle(vertices_2d, buffer_size);
    
    for y in min_y..max_y {  
        for x in min_x..max_x {

            let point = &Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            

            if triangles::is_inside_triangle(vertices_2d, point, super::settings::FRONT_FACE) {
     
                let weights = &triangles::calc_barycentric(vertices_2d, point);

                let pixel_index = (y * buffer_size.x as usize + x) * 4; 


                let input = FragmentPipe {
                    vertex: triangles::mul_barycentric(weights, vertices),
                    uv_mapping: triangles::mul_barycentric(weights, uv_mappings),
                    norm: triangles::mul_barycentric(weights, norms),

                    color: if let Some(colors) = colors { Some(triangles::mul_barycentric(weights, colors)) } else { None },

                    material: Rc::clone(&material),
                    lights,
                };

                let output = shader.fragment_shader(input);

                buffer[pixel_index + 0] = output.x as u8;
                buffer[pixel_index + 1] = output.y as u8;
                buffer[pixel_index + 2] = output.z as u8;
                buffer[pixel_index + 3] = output.w as u8;
            }
        }
    }
}