use crate::graphics::{color::Color, texture::Texture};
use std::{sync::Arc, sync::Mutex};

#[derive(Clone)]
pub struct SpriteRenderer {
    pub texture: Arc<Mutex<dyn Texture>>,
    pub color: Color,
}

unsafe impl Send for SpriteRenderer {}