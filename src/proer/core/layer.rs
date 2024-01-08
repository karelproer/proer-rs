use crate::system::event::Event;
use super::application::Application;
use crate::system::window::Window;
use crate::graphics::renderer::Renderer;

pub trait Layer<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> {
    fn on_create(&mut self, _app: &mut Application<WindowImpl, RendererImpl>) {}
    fn on_update(&mut self, _elapsed_time: std::time::Duration, _app: &mut Application<WindowImpl, RendererImpl>) {}
    fn on_event(&mut self, _e: Event, _app: &mut Application<WindowImpl, RendererImpl>) -> bool { false }
    fn on_destroy(&mut self, _app: &mut Application<WindowImpl, RendererImpl>) {}
}