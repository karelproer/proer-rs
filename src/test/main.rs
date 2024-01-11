extern crate proer;
use proer::system::window::{Window, CursorMode};
use proer::system::event::Event;
use proer::graphics::renderer::Renderer;
use proer::graphics::shader::Shader;
use proer::graphics::texture::*;
use proer::graphics::vertexlayout::VertexAtribute;
use proer::graphics::vertexlayout::VertexAttributeType;
use proer::core::application::Application;
use proer::core::layer::Layer;

use std::boxed::Box;
use nalgebra as na;
use na::{Transform3, Matrix4, Rotation3, Translation3, Scale3};

struct TestLayer {
    time: f32,
}

impl<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> Layer<WindowImpl, RendererImpl> for TestLayer {
    fn on_update(&mut self, elapsed: std::time::Duration, app: &mut Application<WindowImpl, RendererImpl>) {
        app.window().set_cursor_mode(CursorMode::Disabled);
        assert!(app.set_raw_mouse_input(true));
        self.time += elapsed.as_secs_f32();
        let size = app.get_size();
        app.renderer().begin_scene(proer::graphics::color::Color { r: 0, g: 20, b: 0, a: 0 }, size);
        let mut shader = RendererImpl::ShaderType::new(r#"
            #version 330 core
            layout(location = 0) in vec3 a_Pos;
            layout(location = 1) in vec2 a_TexCoord;

            uniform mat4 u_Transform;
            out vec2 v_TexCoord;

            void main() {
                gl_Position = vec4(a_Pos, 1.0) * u_Transform;
                v_TexCoord = a_TexCoord;
            }
        "#, r#"
            #version 330 core
            out vec4 color;
            
            in vec2 v_TexCoord;

            uniform sampler2D texture0;

            void main() {
                color = texture(texture0, v_TexCoord);
            }
        "#).unwrap();
        let t = (Translation3::<f32>::new( self.time.cos() * 0.4, (self.time * 0.2).cos() * 0.5, 0.0) * Rotation3::<f32>::from_euler_angles(0.0, 0.0, self.time)).to_matrix() * Scale3::<f32>::new((self.time * 0.1).tan(), (self.time * 0.1).tan(), 1.0).to_homogeneous();
        let location = shader.get_uniform_location("u_Transform");
        shader.set_uniform_matrix(location, t.into());
        let layout: [VertexAtribute; 2] = [VertexAtribute { name: String::from("Pos"), datatype: VertexAttributeType::Float3, interpolate: true }, VertexAtribute { name: String::from("TexCoord"), datatype: VertexAttributeType::Float2, interpolate: true }];
        
        #[repr(C)]
        struct Vertex([f32; 3], [f32; 2]);
        const VERTICES: [Vertex; 4] =
        [Vertex([-0.5, -0.5, 0.0], [1.0, 1.0]), Vertex([0.5, -0.5, 0.0], [0.0, 1.0]), Vertex([0.5, 0.5, 0.0], [0.0, 0.0]), Vertex([-0.5, 0.5, 0.0], [1.0, 0.0])];
        const INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];
        let image = image::open("image.jpg").unwrap().into_rgba8();
        let texture = [RendererImpl::TextureType::new(image, SamplingMode::Nearset)];
        app.renderer().draw(&VERTICES, &INDICES, &layout, shader, &texture);

        app.renderer().end_scene();
    }

    fn on_event(&mut self, e: Event, _app: &mut Application<WindowImpl, RendererImpl>) -> bool {
        log::info!("event: {:?}", e);
        false
    }
}

fn main() {
    env_logger::init();

    Application::<proer::system::glfw::window::Window, proer::graphics::opengl::renderer::Renderer<proer::system::glfw::window::Window>>::new("proer", (800, 600), vec!(Box::new(TestLayer {time: 0.0})));
}