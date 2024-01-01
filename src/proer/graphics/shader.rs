pub trait Shader {
    fn new(vertex_source: &str, fragment_source: &str) -> Option<Self> where Self: Sized;
}