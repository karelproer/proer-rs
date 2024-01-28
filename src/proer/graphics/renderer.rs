use super::color::Color;
use super::vertexlayout::VertexAtribute;
use super::shader::Shader;
use super::texture::Texture;
use super::renderable::Renderable;
use super::framebuffer::FrameBuffer;
use std::{sync::Arc, sync::Mutex};

pub trait Renderer<Context> {
    type ShaderType: Shader;
    type TextureType: Texture + 'static;
    type RenderableType: Renderable;
    type FrameBufferType: FrameBuffer<TextureType = Self::TextureType>;

    fn new(context: Arc<Mutex<Context>>) -> Self;
    
    fn begin_scene(&mut self, background: &Color, viewport_size: (u32, u32));
    fn begin_scene_framebuffer(&mut self, background: &Color, viewport_size: (u32, u32), framebuffer: &mut Self::FrameBufferType);
    fn end_scene(&mut self);

    fn get_aspectratio(&self) -> f32;

    fn draw<Vertex>(&mut self, vertices: &[Vertex], indices: &[u32], vertex_layout: &[VertexAtribute], shader: &Self::ShaderType, textures: &[Self::TextureType]);
    fn draw_renderable(&mut self, renderable: &Self::RenderableType, shader: &Self::ShaderType, textures: &[Self::TextureType]);
}