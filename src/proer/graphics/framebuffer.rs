use super::texture::Texture;
use std::{rc::Rc};

pub trait FrameBuffer {
    type TextureType: Texture;

    fn new(texture: Rc<Self::TextureType>) -> Self;
}
