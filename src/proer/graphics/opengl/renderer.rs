extern crate gl;
use crate::system::window;
use super::super::color::Color;
use super::vao::Vao;
use super::vbo::Vbo;
use super::ibo::Ibo;
use super::texture::Texture;
use super::shader::ShaderProgram;
use super::super::vertexlayout::VertexAtribute;

pub struct Renderer<Context> {
    context: std::rc::Rc<std::cell::RefCell<Context>>
}

impl<Context: window::OpenGLContext> super::super::renderer::Renderer<Context> for Renderer<Context> {
    fn new(context: std::rc::Rc<std::cell::RefCell<Context>>) -> Self {
        gl::load_with(|s| context.borrow_mut().get_proc_address(s));
        Self {
            context
        }
    }

    fn begin_scene(&mut self, background: Color, viewport_size: (u32, u32)) {
        self.context.borrow_mut().make_current();
        unsafe {
            gl::Viewport(0, 0, viewport_size.0.try_into().unwrap(), viewport_size.1.try_into().unwrap());
            gl::ClearColor(background.r as f32 / 255.0, background.g as f32 / 255.0, background.b as f32 / 255.0, background.a as f32 / 255.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn end_scene(&mut self) {
    }

    type ShaderType = ShaderProgram;
    type TextureType = Texture;

    fn draw<Vertex>(&mut self, vertices: &[Vertex], indices: &[u32], vertex_layout: &[VertexAtribute], shader: &Self::ShaderType, textures: &[Self::TextureType]) {
        unsafe {
            let vao = Vao::new();
            vao.bind();
            let mut vbo = Vbo::new();
            vbo.bind();
            vbo.data(vertices);
            let mut ibo = Ibo::new();
            ibo.bind();
            ibo.data(indices);
            
            vao.set_vertex_layout(&vertex_layout);

            for (n, t) in textures.iter().enumerate() {
                t.bind(n.try_into().unwrap());
            }
            
            shader.bind();

            gl::DrawElements(gl::TRIANGLES, indices.len().try_into().unwrap(), gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}