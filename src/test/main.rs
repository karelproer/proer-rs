extern crate proer;
use proer::system::window::{Window, CursorMode};
use proer::system::event::Event;
use proer::graphics::{renderer::Renderer, color::Color};
use proer::graphics::shader::Shader;
use proer::graphics::texture::*;
use proer::graphics::vertexlayout::VertexAtribute;
use proer::graphics::vertexlayout::VertexAttributeType;
use proer::core::application::Application;
use proer::core::layer::Layer;
use proer::ecs::{scene::Scene, systems::{scenerenderer::{SceneRenderer, render_scene_system}, simplescenerenderer::SimpleSceneRenderer}};
use proer::ecs::components::{spriterenderer::SpriteRenderer, transform::Transform};
use proer::*;

use std::{boxed::Box, rc::Rc, cell::RefCell};
use std::borrow::BorrowMut;
use std::{sync::Arc, sync::Mutex};
use nalgebra as na;
use na::{Transform3, Matrix4, Rotation3, Translation3, Scale3, Similarity3, UnitQuaternion, Vector3};
use legion::{IntoQuery, Schedule};

struct TestLayer<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> {
    time: f32,
    scene: Option<Scene>,
    scenerenderer: Option<Arc<Mutex<SimpleSceneRenderer<WindowImpl, RendererImpl>>>>,
}

impl<WindowImpl: Window + 'static, RendererImpl: Renderer<WindowImpl> + 'static> Layer<WindowImpl, RendererImpl> for TestLayer<WindowImpl, RendererImpl> {
    fn on_create(&mut self, app: &mut Application<WindowImpl, RendererImpl>) {
        let image = image::open("image.jpg").unwrap().into_rgba8();
        self.scene = Some(Scene::new());
        self.scene.as_mut().unwrap().create_entity((SpriteRenderer {texture: Arc::new(Mutex::new(RendererImpl::TextureType::new(image, SamplingMode::Nearset))), color: Color {r: 255, g: 255, b: 255, a: 255}}, Transform {transform: Similarity3::from_parts(Translation3::new(0.5, 0.5, 0.0), UnitQuaternion::from_euler_angles(0.0, 0.0, 1.0), 2.0)}));
        self.scenerenderer = Some(Arc::new(Mutex::new(SimpleSceneRenderer::new(app.get_renderer_pointer()))))
    }

    fn on_update(&mut self, elapsed: std::time::Duration, app: &mut Application<WindowImpl, RendererImpl>) {
        let size = app.get_size();
        app.renderer().begin_scene(Color {r: 0, g: 200, b: 0, a: 255}, size);

        let mut schedule = Schedule::builder().add_thread_local(render_scene_system(self.scenerenderer.as_mut().unwrap().clone())).build();
        self.scene.as_mut().unwrap().run(&mut schedule);

        app.renderer().end_scene();
    }

    fn on_event(&mut self, e: Event, _app: &mut Application<WindowImpl, RendererImpl>) -> bool {
        log::info!("event: {:?}", e);
        false
    }
}

fn main() {
    env_logger::init();

    Application::<proer::system::glfw::window::Window, proer::graphics::opengl::renderer::Renderer<proer::system::glfw::window::Window>>::new("proer", (800, 600), vec!(Box::new(TestLayer::<proer::system::glfw::window::Window, proer::graphics::opengl::renderer::Renderer<proer::system::glfw::window::Window>> {time: 0.0, scene: None, scenerenderer: None})));
}