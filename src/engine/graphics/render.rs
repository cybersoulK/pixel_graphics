use std::{rc::Rc, time::Duration};

use glam::{Vec2, Mat4, Vec3Swizzles};


use super::super::{
    {Shader, VertexPipe, FragmentPipe, CorePipe},
    {Camera, DrawableObject, Light},
    {ModelIterator, MeshIterator}};

use super::triangles;



pub fn render_update(buffer: &mut [u8], z_buffer: &mut [f32], buffer_size: Vec2, camera: &Camera, drawables: &Vec<DrawableObject>, lights: &Vec<Light>, time: Duration) {


    buffer.fill(0);
    z_buffer.fill(f32::MAX);


    let vp_matrix = camera.get_vp_matrix(buffer_size);

    for obj in drawables {

        let model_matrix = obj.transform.matrix;

        let model = Rc::clone(&obj.model);


        for (mesh, material) in ModelIterator::new(&model) {
            
            let shader = material.get_shader();
            
            for (vertices, uv_mappings, norms, face_id) in MeshIterator::new(&mesh) {
                
                let colors = [None; 3];


                let core_pipe = CorePipe::new_bundle(vertices, uv_mappings, norms, colors); 

                let mut vertex_pipe = VertexPipe {
                    time,
                };

                let core_pipe = execute_vertex_shader(&shader, core_pipe, model_matrix, &mut vertex_pipe, face_id);


                let fragment_pipe = FragmentPipe {
                    material: Rc::clone(&material),
                    lights,
                    time,
                };
                
                execute_fragment_shader(&shader, core_pipe, vp_matrix, &fragment_pipe, buffer, z_buffer, buffer_size, camera);
            }
        }
    }
}

fn execute_vertex_shader(shader: &Rc<dyn Shader>, core_pipe: [CorePipe; 3], model_matrix: Mat4, vertex_pipe: &mut VertexPipe, face_id: usize) -> [CorePipe; 3] {

    [0, 1, 2].map(|vertex_id| {
        let mut core_pipe = shader.vertex_shader(core_pipe[vertex_id], &vertex_pipe, vertex_id, face_id);

        core_pipe.vertex = model_matrix.project_point3(core_pipe.vertex);
        core_pipe
    })
}

fn execute_fragment_shader(shader: &Rc<dyn Shader>, core_pipe: [CorePipe; 3], vp_matrix: Mat4, fragment_pipe: &FragmentPipe, buffer: &mut [u8], z_buffer: &mut [f32], buffer_size: Vec2, camera: &Camera) {

    let vertices = core_pipe.map(|c| c.vertex);
    let uv_mappings = core_pipe.map(|c| c.uv_mapping);
    let norms = core_pipe.map(|c| c.norm);

    let colors = CorePipe::transform_option(core_pipe.map(|v| v.color));
 

    let vertices_2d_3 = vertices.map(|vertex| {
        vp_matrix.inverse().transform_point3(vertex)
    });

    let vertices_2d = &vertices_2d_3.map(|vertex| {
        vertex.xy()
    });


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
                let vertex_2d = triangles::mul_barycentric(weights, vertices_2d_3);

                let output = shader.fragment_shader(core_pipe, &fragment_pipe, vertex_2d);


                let depth_index = y * buffer_size.x as usize + x; 
                
                if vertex_2d.z < z_buffer[depth_index] && vertex_2d.z >= camera.get_near() {
                    buffer[pixel_index + 0] = output.x as u8;
                    buffer[pixel_index + 1] = output.y as u8;
                    buffer[pixel_index + 2] = output.z as u8;
                    buffer[pixel_index + 3] = output.w as u8;

                    z_buffer[depth_index] = vertex_2d.z;
                }
            }
        }
    }
}