use crate::graphics::{color::Color, texture::Texture};
use std::{rc::Rc, sync::Arc, sync::Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct SpriteRenderer {
    pub texture: Arc<Mutex<dyn Texture>>,
    pub color: Color,
}

unsafe impl Send for SpriteRenderer {}