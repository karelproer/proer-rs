use nalgebra::{Similarity3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub transform: Similarity3<f32>,
}