use super::vao::Vao;
use super::vbo::Vbo;
use super::ibo::Ibo;
use super::super::vertexlayout::VertexAtribute;

pub struct Renderable {
    vao: Vao,
    indexcount: usize,
}

impl Renderable {
    pub fn bind(&self) {
        self.vao.bind();
    }

    pub fn draw(&self) {
        self.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indexcount.try_into().unwrap(), gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

impl super::super::renderable::Renderable for Renderable {
    fn new<Vertex>(vertices: &[Vertex], indices: &[u32], vertex_layout: &[VertexAtribute]) -> Self {
        let vao = Vao::new();
        vao.bind();
        let mut ibo = Ibo::new();
        ibo.data(indices);
        let mut vbo = Vbo::new();
        vbo.data(vertices);
        vao.set_vertex_layout(vertex_layout);
        Vao::unbind();
        Self { vao, indexcount: indices.len() }
    }
}