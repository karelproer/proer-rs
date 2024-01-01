use super::event;


pub trait Window {
    type Platform;
    fn new(platform: &mut Self::Platform, size: (u32,  u32), title: &str) -> Self;
    fn open(&mut self, platform: &mut Self::Platform) -> bool;
    fn update(&mut self, platform: &mut Self::Platform);
    fn get_event(&mut self) -> Option<event::Event>;
}

pub trait OpenGLContext {
    fn init(&mut self) {}
    fn make_current(&mut self);
    fn get_proc_address(&mut self, procname: &str) -> *const std::os::raw::c_void;
}