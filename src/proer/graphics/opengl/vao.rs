use super::super::vertexlayout::VertexAtribute;
use super::super::vertexlayout::VertexAttributeType;
use gl::types::*;

pub struct Vao {
    id: u32,
}

impl Vao {
    pub fn new() -> Self {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        assert_ne!(vao, 0);
        Self { id: vao }
    }
    
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn vertex_attribute_type_to_opengl(t: VertexAttributeType) -> GLenum {
        match t {
            VertexAttributeType::Float => gl::FLOAT,
            VertexAttributeType::Float2 => gl::FLOAT,
            VertexAttributeType::Float3 => gl::FLOAT,
            VertexAttributeType::Float4 => gl::FLOAT,
            VertexAttributeType::Int => gl::INT,
        }
    }

    pub fn set_vertex_layout(&self, layout: &[VertexAtribute]) {
        self.bind();
        let total_size: u32 = layout.iter().map(|x| x.datatype.size()).sum();
        let mut size = 0;
        for (n, attr) in layout.iter().enumerate() {
            unsafe {
                gl::VertexAttribPointer(n.try_into().unwrap(), attr.datatype.amount().try_into().unwrap(), Self::vertex_attribute_type_to_opengl(attr.datatype), 0, total_size.try_into().unwrap(), size as *const _);
                gl::EnableVertexAttribArray(n.try_into().unwrap());
                size += attr.datatype.size();
            }
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}