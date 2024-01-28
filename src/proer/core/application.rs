use crate::system::window::Window;
use crate::graphics::renderer::Renderer;
use crate::system::platform::Platform;
use crate::utils::time::frametimer::FrameTimer;
use crate::system::event::Event;
use super::layer::Layer;

use std::{vec::Vec, boxed::Box, sync::{Arc, Mutex, MutexGuard}};


pub struct Application<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> {
    layers: Vec<Box<dyn Layer<WindowImpl, RendererImpl>>>,
    renderer: Arc<Mutex<RendererImpl>>,
    window: Arc<Mutex<WindowImpl>>,
    platform: WindowImpl::Platform,
    running: bool,
    frame_timer: FrameTimer,
    size: (u32, u32),
    cursor_pos: (f64, f64),
}

impl<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> Application<WindowImpl, RendererImpl> {
    pub fn new(name: &str, size: (u32, u32)) -> Self{
        let mut platform = WindowImpl::Platform::new();
        let window = Arc::new(Mutex::new(WindowImpl::new(&mut platform, size, name)));
        let renderer = Arc::new(Mutex::new(RendererImpl::new(window.clone())));
        Self {
            renderer,
            platform,
            window,
            layers: Vec::new(),
            running: true,
            frame_timer: FrameTimer::new(),
            size,
            cursor_pos: ((0.0, 0.0))
        }
    }

    pub fn add_layer<T: Layer<WindowImpl, RendererImpl> + 'static>(&mut self) {
        let mut layers_borrowed = std::mem::replace(&mut self.layers, vec!());
        layers_borrowed.push(Box::new(T::create(self)));
        self.layers = layers_borrowed;
    }

    pub fn run(&mut self) {
        while self.running {
            let dur = self.frame_timer.frame();

            while let Some(e) = { let x = self.window.lock().unwrap().get_event(); x } {
                match e {
                    Event::Resize(s) => {
                        self.size = s;
                    }
                    Event::Close => {
                        self.running = false;
                    }
                    Event::CursorMove(pos) => {
                        self.cursor_pos = pos;
                    }
                    _ => {}
                }

                let mut layers_borrowed = std::mem::replace(&mut self.layers, vec!());
                for l in &mut layers_borrowed {
                    if l.on_event(e, self) {
                        break;
                    }
                }
                self.layers = layers_borrowed;        
            }
            let mut layers_borrowed = std::mem::replace(&mut self.layers, vec!());
            for l in &mut layers_borrowed {
                l.on_update(dur, self);
            }
            self.layers = layers_borrowed;
            self.window.lock().unwrap().update(&mut self.platform);
        }

        let mut layers_borrowed = std::mem::replace(&mut self.layers, vec!());
        for l in &mut layers_borrowed {
            l.on_destroy(self);
        }
        self.layers = layers_borrowed;
    }

    pub fn window(&self) -> MutexGuard<'_, WindowImpl> {
        self.window.lock().unwrap()
    }

    pub fn renderer(&mut self) -> MutexGuard<'_, RendererImpl> {
        self.renderer.lock().unwrap()
    }

    pub fn get_renderer_pointer(&mut self) -> Arc<Mutex<RendererImpl>> {
        self.renderer.clone()
    }

    pub fn close(&mut self) {
        self.running = false;
    }

    pub fn get_fps(&self) -> f32 {
        self.frame_timer.fps()
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    pub fn set_raw_mouse_input(&mut self, raw: bool) -> bool {
        self.window.lock().unwrap().set_raw_mouse_input(raw, &mut self.platform)
    }

    pub fn get_cursor_pos(&mut self) -> (f64, f64) {
        self.cursor_pos
    }
}