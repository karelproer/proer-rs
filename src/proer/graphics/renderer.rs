use super::color::Color;
use super::vertexlayout::VertexAtribute;
use super::shader::Shader;
use super::texture::Texture;

pub trait Renderer<Context> {
    type ShaderType: Shader;
    type TextureType: Texture;
    
    fn new(context: std::rc::Rc<std::cell::RefCell<Context>>) -> Self;
    
    fn begin_scene(&mut self, background: Color, viewport_size: (u32, u32));
    fn end_scene(&mut self);

    fn draw<Vertex>(&mut self, vertices: &[Vertex], indices: &[u32], vertex_layout: &[VertexAtribute], shader: &Self::ShaderType, textures: &[Self::TextureType]);
}