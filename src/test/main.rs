extern crate proer;
use proer::system::platform::Platform;
use proer::system::window::Window;
use proer::graphics::renderer::Renderer;
use proer::graphics::shader::Shader;
use proer::graphics::texture::*;
use proer::graphics::vertexlayout::VertexAtribute;
use proer::graphics::vertexlayout::VertexAttributeType;

fn main() {
    env_logger::init();

    let mut platform = proer::system::glfw::platform::Platform::new();
    let size = std::cell::RefCell::new((800, 600));
    let window = std::rc::Rc::new(std::cell::RefCell::new(proer::system::glfw::window::Window::new(&mut platform, size.borrow().clone(), "proer")));

    let mut renderer = proer::graphics::opengl::renderer::Renderer::new(window.clone());
    
    while window.borrow_mut().open(&mut platform) {
        while let Some(e) = window.borrow_mut().get_event() {
            match e {
                proer::system::event::Event::Resize(s) => {
                    *size.borrow_mut() = s;
                }
                _ => {}
            }
        }

        renderer.begin_scene(proer::graphics::color::Color { r: 0, g: 20, b: 0, a: 0 }, size.borrow().clone());
        let shader = proer::graphics::opengl::shader::ShaderProgram::new(r#"
            #version 330 core
            layout(location = 0) in vec3 a_Pos;
            layout(location = 1) in vec2 a_TexCoord;

            out vec2 v_TexCoord;

            void main() {
                gl_Position = vec4(a_Pos, 1.0);
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
        let layout: [VertexAtribute; 2] = [VertexAtribute { name: String::from("Pos"), datatype: VertexAttributeType::Float3, interpolate: true }, VertexAtribute { name: String::from("TexCoord"), datatype: VertexAttributeType::Float2, interpolate: true }];
        #[repr(C)]
        struct Vertex([f32; 3], [f32; 2]);
        const VERTICES: [Vertex; 4] =
        [Vertex([-0.5, -0.5, 0.0], [1.0, 1.0]), Vertex([0.5, -0.5, 0.0], [0.0, 1.0]), Vertex([0.5, 0.5, 0.0], [0.0, 0.0]), Vertex([-0.5, 0.5, 0.0], [1.0, 0.0])];
        const INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];
        let image = image::open(env!("CARGO_MANIFEST_DIR").to_owned() + "/" + "image.jpg").unwrap().into_rgba8();
        let texture = [proer::graphics::opengl::texture::Texture::new(image, SamplingMode::Linear)];
        renderer.draw(&VERTICES, &INDICES, &layout, shader, &texture);

        renderer.end_scene();
        window.borrow_mut().update(&mut platform);
    }

    platform.deinit();
}