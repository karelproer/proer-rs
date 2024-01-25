use super::texture::Texture;
use std::rc::Rc;

pub struct FrameBuffer {
    id: u32,
    _color: Rc<Texture>,
}

impl FrameBuffer {
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl super::super::framebuffer::FrameBuffer for FrameBuffer {
    type TextureType = Texture;
    
    fn new(texture: Rc<Texture>) -> Self {
        unsafe {
            let mut fbo = 0;
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            texture.bind(0);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture.id, 0);  
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            Self {id: fbo, _color: texture}
        }
    }
}


impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        }
    }
}