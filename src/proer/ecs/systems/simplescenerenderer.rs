use super::scenerenderer::SceneRenderer;
use crate::system::window::Window;
use crate::graphics::renderer::Renderer;
use crate::graphics::vertexlayout::VertexAtribute;
use crate::graphics::vertexlayout::VertexAttributeType;
use crate::graphics::shader::Shader;
use crate::graphics::renderable::Renderable;
use super::super::components::{spriterenderer::SpriteRenderer, transform::Transform};
use std::{rc::Rc, cell::RefCell};
use std::{sync::Arc, sync::Mutex};

pub struct SimpleSceneRenderer<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> {
    renderer: Arc<Mutex<RendererImpl>>,
    shader: RendererImpl::ShaderType,
    renderable: RendererImpl::RenderableType,
}

#[repr(C)]
struct Vertex([f32; 3], [f32; 2]);

impl<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> SimpleSceneRenderer<WindowImpl, RendererImpl> {
    pub fn new(renderer: Arc<Mutex<RendererImpl>>) -> Self {
        let shader = RendererImpl::ShaderType::new(r#"
            #version 450 core
            layout(location = 0) in vec3 a_Pos;
            layout(location = 1) in vec2 a_TexCoord;

            layout(location = 1) uniform mat4 u_Transform;
            out vec2 v_TexCoord;
            
            void main() {
                gl_Position = vec4(a_Pos, 1.0) * u_Transform;
                v_TexCoord = a_TexCoord;
            }
            "#, r#"
            #version 450 core
            out vec4 color;
            
            in vec2 v_TexCoord;
            
            layout(location = 2) uniform vec4 u_Color;
            layout(binding = 0) uniform sampler2D texture0;

            void main() {
                color = texture(texture0, v_TexCoord) * u_Color;
            }
        "#).unwrap();

        const VERTICES: [Vertex; 4] =
        [Vertex([-0.5, -0.5, 0.0], [1.0, 1.0]), Vertex([0.5, -0.5, 0.0], [0.0, 1.0]), Vertex([0.5, 0.5, 0.0], [0.0, 0.0]), Vertex([-0.5, 0.5, 0.0], [1.0, 0.0])];
        const INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];
        let LAYOUT: [VertexAtribute; 2] = [VertexAtribute { name: String::from("a_Pos"), datatype: VertexAttributeType::Float3, interpolate: true }, VertexAtribute { name: String::from("a_TexCoord"), datatype: VertexAttributeType::Float2, interpolate: true }];
        let renderable = RendererImpl::RenderableType::new(&VERTICES, &INDICES, &LAYOUT);

        Self {
            renderer,
            shader,
            renderable,
        }
    }
}

impl<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> SceneRenderer for SimpleSceneRenderer<WindowImpl, RendererImpl> {
    fn render(&mut self, spriterenderer: &SpriteRenderer, transform: &Transform) {
        self.shader.set_uniform_matrix(1, transform.transform.into());
        self.shader.set_uniform_float4(2, spriterenderer.color.as_vec4());
        // todo: make sort of safeish
        let texture =  unsafe { &mut *(Arc::into_raw(spriterenderer.texture.clone()) as *mut Mutex::<RendererImpl::TextureType>) };
        (*self.renderer).lock().unwrap().draw_renderable(&self.renderable, &self.shader,  std::slice::from_ref(&texture.lock().unwrap()))
    }
}