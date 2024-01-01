extern crate glfw;
use super::super::platform;

pub struct Platform {
    glfw: glfw::Glfw,
}

impl Platform {
    pub fn get_glfw(&mut self) -> &mut glfw::Glfw {
        &mut self.glfw
    }
}

impl platform::Platform for Platform {
    fn new() -> Self {
        Self{
            glfw: glfw::init(error_callback).unwrap(),
        }
    }
}

fn error_callback(err: glfw::Error, description: String) {
    log::error!("GLFW error {:?}: {:?}", err, description);
}