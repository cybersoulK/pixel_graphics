use std::{rc::Rc, time::Duration};

use glam::{Vec2, Vec3Swizzles};


use super::super::{
    {Shader, VertexPipe, FragmentPipe, CorePipe},
    {Camera, DrawableObject, Light},
    {ModelIterator, MeshIterator}};

use super::triangles;



pub fn render_update(buffer: &mut [u8], buffer_size: Vec2, camera: &Rc<Camera>, drawables: &Vec<Rc<DrawableObject>>, lights: &Vec<Rc<Light>>, time: Duration) {


    buffer.fill(0);


    let vp_matrix = camera.get_vp_matrix();

    for obj in drawables {

        let mvp_matrix = obj.transform.matrix * vp_matrix;

        let model = Rc::clone(&obj.model);


        for (mesh, material) in ModelIterator::new(&model) {
            
            let shader = material.get_shader();
            
            for (vertices, uv_mappings, norms) in MeshIterator::new(&mesh) {
                
                let colors = [None; 3];


                let core_pipe = CorePipe::new_bundle(vertices, uv_mappings, norms, colors); 

                let mut vertex_pipe = VertexPipe {
                    mvp_matrix,
                    time,
                };

                let core_pipe = execute_vertex_shader(&shader, core_pipe, &mut vertex_pipe);


                let fragment_pipe = FragmentPipe {
                    material: Rc::clone(&material),
                    lights,
                    time,
                };
                
                execute_fragment_shader(&shader, buffer, buffer_size, core_pipe, &fragment_pipe);
            }
        }
    }
}

fn execute_vertex_shader(shader: &Rc<dyn Shader>, core_pipe: [CorePipe; 3], vertex_pipe: &mut VertexPipe) -> [CorePipe; 3] {

    [0, 1, 2].map(|i| {
        shader.vertex_shader(i, core_pipe[i], &vertex_pipe)
    })
}

fn execute_fragment_shader(shader: &Rc<dyn Shader>, buffer: &mut [u8], buffer_size: Vec2, core_pipe: [CorePipe; 3], fragment_pipe: &FragmentPipe,) {

    let vertices = core_pipe.map(|c| c.vertex);
    let uv_mappings = core_pipe.map(|c| c.uv_mapping);
    let norms = core_pipe.map(|c| c.norm);

    let colors = CorePipe::transform_option(core_pipe.map(|v| v.color));


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


                let core_pipe =  CorePipe {
                    vertex: triangles::mul_barycentric(weights, vertices),
                    uv_mapping: triangles::mul_barycentric(weights, uv_mappings),
                    norm: triangles::mul_barycentric(weights, norms),

                    color: if let Some(colors) = colors { Some(triangles::mul_barycentric(weights, colors)) } else { None },
                };

                let output = shader.fragment_shader(core_pipe, &fragment_pipe);

                buffer[pixel_index + 0] = output.x as u8;
                buffer[pixel_index + 1] = output.y as u8;
                buffer[pixel_index + 2] = output.z as u8;
                buffer[pixel_index + 3] = output.w as u8;
            }
        }
    }
}