extern crate proer;
use proer::system::window::{Window};
use proer::system::event::Event;
use proer::graphics::{renderer::Renderer, color::Color};
use proer::graphics::texture::*;
use proer::core::application::Application;
use proer::core::layer::Layer;
use proer::ecs::{scene::Scene, systems::{scenerenderer::{render_scene_system}, simplescenerenderer::SimpleSceneRenderer}};
use proer::ecs::components::{spriterenderer::SpriteRenderer, transform::Transform};

use std::{sync::Arc, sync::Mutex};
use nalgebra as na;
use na::{Translation3, Similarity3, UnitQuaternion };
use legion::{Schedule};

struct TestLayer<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> {
    time: f32,
    scene: Scene,
    scenerenderer: Arc<Mutex<SimpleSceneRenderer<WindowImpl, RendererImpl>>>,
}

impl<WindowImpl: Window + 'static, RendererImpl: Renderer<WindowImpl> + 'static> Layer<WindowImpl, RendererImpl> for TestLayer<WindowImpl, RendererImpl> {
    fn create(app: &mut Application<WindowImpl, RendererImpl>) -> Self {
        let image = image::open("image.jpg").unwrap().into_rgba8();
        let mut scene = Scene::new();
        scene.create_entity((SpriteRenderer {texture: Arc::new(Mutex::new(RendererImpl::TextureType::new(image, SamplingMode::Nearset))), color: Color {r: 255, g: 255, b: 255, a: 255}}, Transform {transform: Similarity3::from_parts(Translation3::new(0.5, 0.5, 0.0), UnitQuaternion::from_euler_angles(0.0, 0.0, 1.0), 2.0)}));
        let scenerenderer = Arc::new(Mutex::new(SimpleSceneRenderer::new(app.get_renderer_pointer())));
        Self { time: 0.0, scene, scenerenderer }
    }

    fn on_update(&mut self, elapsed: std::time::Duration, app: &mut Application<WindowImpl, RendererImpl>) {
        self.time += elapsed.as_secs_f32();
        let size = app.get_size();
        app.renderer().begin_scene(&(Color {r: 0, g: 200, b: 0, a: 255}), size);

        let mut schedule = Schedule::builder().add_thread_local(render_scene_system(self.scenerenderer.clone())).build();
        self.scene.run(&mut schedule);

        app.renderer().end_scene();
    }

    fn on_event(&mut self, e: Event, _app: &mut Application<WindowImpl, RendererImpl>) -> bool {
        log::info!("event: {:?}", e);
        false
    }
}

fn main() {
    env_logger::init();

    let mut app = Application::<proer::system::glfw::window::Window, proer::graphics::opengl::renderer::Renderer<proer::system::glfw::window::Window>>::new("proer", (800, 600));
    app.add_layer::<TestLayer<proer::system::glfw::window::Window, proer::graphics::opengl::renderer::Renderer<proer::system::glfw::window::Window>>>();
    app.run();
}