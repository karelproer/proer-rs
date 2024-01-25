#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub active: bool,
    pub fixed_aspectratio: bool,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}