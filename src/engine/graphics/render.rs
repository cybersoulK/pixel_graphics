use std::rc::Rc;

use glam::Vec2;

use super::super::{
    {RenderingSettings, ShaderPipe},
    {Camera, DrawableObject, Light},
    {ModelIterator, MeshIterator}};



pub fn render_update(
    buffer: &mut [u8],
    buffer_size: Vec2,
    settings: &RenderingSettings,
    camera: &Rc<Camera>,
    drawables: &Vec<Rc<DrawableObject>>,
    lights: &Vec<Rc<Light>>){


    buffer.fill(0);

    for obj in drawables {

        let model = Rc::clone(&obj.model);
        
        for (mesh, material) in ModelIterator::new(&model) {

            let shader = material.get_shader();
            
            for (vertices, uv_textures, norms) in MeshIterator::new(&mesh) {

                let shader_pipe = ShaderPipe {
                    vp_matrix: camera.get_vp_matrix(),
                    model_matrix,

                    vertices,
                    uv_textures,
                    norms,

                    material,
                    lights,
                };

                shader.execute(&shader_pipe);
            }
        }
    }
}