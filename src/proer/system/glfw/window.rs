extern crate glfw;
use super::super::window;
use super::platform;
use super::super::event;
use glfw::Context;

pub struct Window {
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl window::Window for Window {
    type Platform = platform::Platform;

    fn new(platform: &mut Self::Platform, size: (u32, u32), title: &str) -> Self {
        let (mut window, events) = platform.get_glfw().create_window(size.0, size.1, title, glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");
        log::info!("Created GLFW window.");
        window.set_all_polling(true);
        Self {
            window,
            events,
        }
    }

    fn open(&mut self, _platform: &mut Self::Platform) -> bool {
        !self.window.should_close()
    }

    fn update(&mut self, platform: &mut Self::Platform) {
        self.window.swap_buffers();
        platform.get_glfw().poll_events();
    }

    fn get_event(&mut self) -> Option<event::Event> {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Size(width, height) => return Some(event::Event::Resize((width.try_into().unwrap(), height.try_into().unwrap()))),
                glfw::WindowEvent::Close => return Some(event::Event::Close),
                _ => {},
            }
        }
        None
    }
}

impl window::OpenGLContext for Window {
    fn make_current(&mut self) {
        self.window.make_current();
    }

    fn get_proc_address(&mut self, procname: &str) -> *const std::os::raw::c_void {
        self.window.get_proc_address(procname) as *const std::os::raw::c_void
    }
}