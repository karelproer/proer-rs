use super::vertexlayout::VertexAtribute;

pub trait Renderable {
    fn new<Vertex>(vertices: &[Vertex], indices: &[u32], vertex_layout: &[VertexAtribute]) -> Self;
}