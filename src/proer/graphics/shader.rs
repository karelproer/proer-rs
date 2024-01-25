pub trait Shader {
    fn new(vertex_source: &str, fragment_source: &str) -> Option<Self> where Self: Sized;

    fn set_uniform_matrix(&mut self, location: u32, value: nalgebra::Matrix4<f32>);
    fn set_uniform_float(&mut self, location: u32, value: f32);
    fn set_uniform_float2(&mut self, location: u32, value: nalgebra::Vector2<f32>);
    fn set_uniform_float3(&mut self, location: u32, value: nalgebra::Vector3<f32>);
    fn set_uniform_float4(&mut self, location: u32, value: nalgebra::Vector4<f32>);
    fn get_uniform_location(&mut self, name: &str) -> u32;
}