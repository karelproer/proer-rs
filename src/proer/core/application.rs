use crate::system::window::Window;
use crate::graphics::renderer::Renderer;
use crate::system::platform::Platform;
use crate::utils::time::frametimer::FrameTimer;
use crate::system::event::Event;
use super::layer::Layer;

use std::{vec::Vec, boxed::Box, cell::RefCell};


pub struct Application<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> {
    renderer: RendererImpl,
    platform: WindowImpl::Platform,
    window: std::rc::Rc<RefCell<WindowImpl>>,
    layers: Vec<Box<dyn Layer<WindowImpl, RendererImpl>>>,
    running: bool,
    frame_timer: FrameTimer,
    size: (u32, u32),
}

impl<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> Application<WindowImpl, RendererImpl> {
    pub fn new(name: &str, size: (u32, u32), layers: Vec<Box<dyn Layer<WindowImpl, RendererImpl>>>) {
        let mut platform = WindowImpl::Platform::new();
        let window = std::rc::Rc::new(RefCell::new(WindowImpl::new(&mut platform, size, name)));
        let renderer = RendererImpl::new(window.clone());
        Self {
            renderer,
            platform,
            window,
            layers,
            running: true,
            frame_timer: FrameTimer::new(),
            size,
        }.run();
    }

    fn run(&mut self) {
        let mut layers_borrowed = std::mem::replace(&mut self.layers, vec!());
        for l in &mut layers_borrowed {
            l.on_create(self);
        }
        self.layers = layers_borrowed;

        while self.running {
            let dur = self.frame_timer.frame();

            while let Some(e) = { let x = self.window.borrow_mut().get_event(); x } {
                match e {
                    Event::Resize(s) => {
                        self.size = s;
                    }
                    Event::Close => {
                        self.running = false;
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

            self.window.borrow_mut().update(&mut self.platform);
        }

        let mut layers_borrowed = std::mem::replace(&mut self.layers, vec!());
        for l in &mut layers_borrowed {
            l.on_destroy(self);
        }
        self.layers = layers_borrowed;
    }

    pub fn window(&self) -> std::cell::RefMut<'_, WindowImpl> {
        self.window.borrow_mut()
    }

    pub fn renderer(&mut self) -> &mut RendererImpl {
        &mut self.renderer
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
        self.window.borrow_mut().set_raw_mouse_input(raw, &mut self.platform)
    }
}