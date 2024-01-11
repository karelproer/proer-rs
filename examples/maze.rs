extern crate proer;
use proer::system::window::Window;
use proer::system::event::{Event, Key, Action};
use proer::graphics::renderer::Renderer;
use proer::graphics::shader::Shader;
use proer::graphics::texture::*;
use proer::graphics::vertexlayout::VertexAtribute;
use proer::graphics::vertexlayout::VertexAttributeType;
use proer::core::application::Application;
use proer::core::layer::Layer;

use std::{boxed::Box, vec::Vec};
use nalgebra as na;
use na::{Transform3, Matrix4, Rotation3, Translation3, Scale3, Vector2};

struct MazeLayer<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> {
    player_pos: Vector2<f32>,
    player_angle: f32,
    shader: Option<RendererImpl::ShaderType>,
    textures: Vec<RendererImpl::TextureType>,
    map: Option<image::RgbaImage>,
    key_states: [bool; 4],
    fov: f32,
}

#[repr(C)]
struct Vertex([f32; 2]);

impl<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> Layer<WindowImpl, RendererImpl> for FpsLayer<WindowImpl, RendererImpl> {
    fn on_create(&mut self, app: &mut Application<WindowImpl, RendererImpl>) {
        self.shader = RendererImpl::ShaderType::new(r#"
            #version 330 core
            layout(location = 0) in vec2 a_Pos;
            out vec2 v_Pos;

            void main() {
                gl_Position = vec4(a_Pos, 0.0, 1.0);
                v_Pos = a_Pos;
            }
        "#, r#"
            #version 460 core
            out vec4 color;
            in vec2 v_Pos;

            layout(location = 0) uniform vec2 u_PlayerPos;
            layout(location = 1) uniform float u_PlayerAngle;
            layout(location = 2) uniform float u_Fov;
            
            layout(binding = 0) uniform sampler2D u_Map;
            layout(binding = 1) uniform sampler2D u_Wall;

            float map(float value, float min1, float max1, float min2, float max2) {
                return min2 + (value - min1) * (max2 - min2) / (max1 - min1);
            }

            void main() {
                float maxDepth = 20;

                float rayAngle = u_PlayerAngle + atan(v_Pos.x * 0.5) * u_Fov;
                vec2 rayDirection = vec2(sin(rayAngle), cos(rayAngle));// / cos(atan(v_Pos.x * 0.5) * u_Fov);
                bool hitWall = false;
                float distance = 0.0;
                vec2 hitPos;
                while(!hitWall && distance <= maxDepth) {
                    distance += 0.001;

                    vec2 test = u_PlayerPos + rayDirection * distance;
                    if(texture(u_Map, test / textureSize(u_Map, 0)).x > 0.5) {
                        hitWall = true;
                        hitPos = u_PlayerPos + rayDirection * distance;
                    }
                }

                float height = 1.0 / (distance * cos(atan(v_Pos.x * 0.5) * u_Fov)) * 0.8;

                if(v_Pos.y <= height && v_Pos.y >= -height)
                {
                    float distanceGrayness = distance / maxDepth * 0.8;
                    vec2 hitInWall = hitPos - vec2(int(hitPos.x), int(hitPos.y));
                    float wallTextureX = 100;
                    if(hitInWall.x > 0.999)
                        wallTextureX = hitInWall.y;
                    else if(hitInWall.x < 0.001)
                        wallTextureX = hitInWall.y;
                    else if(hitInWall.y > 0.999)
                        wallTextureX = hitInWall.x;
                    else if(hitInWall.y < 0.001)
                        wallTextureX = hitInWall.x;
                        
                    color = vec4(0.3, 0.3, 0.3, 1.0) * distanceGrayness + texture(u_Wall, vec2(wallTextureX, map(v_Pos.y, -height, height, 0.0, 1.0))) * (1.0 - distanceGrayness);
                    if(wallTextureX == 100)
                        color = vec4(1.0, 0.0, 0.0, 0.0);
                }
                else if(v_Pos.y < 0)
                {
                    float brightness = -v_Pos.y;
                    color = vec4(brightness * 0.2, brightness * 0.4, 0.0, 1.0);
                }
                else
                    color = vec4(0.0, 0.1, 0.8, 1.0);
            }
        "#);

        let image = image::open("examples/map.png").unwrap().into_rgba8();
        self.textures.push(RendererImpl::TextureType::new(image.clone(), SamplingMode::Nearset));
        self.map = Some(image);
        let wallimage = image::open("examples/wall.png").unwrap().into_rgba8();
        self.textures.push(RendererImpl::TextureType::new(wallimage.clone(), SamplingMode::Linear));
    }

    fn on_update(&mut self, elapsed: std::time::Duration, app: &mut Application<WindowImpl, RendererImpl>) {
        let size = app.get_size();
        app.renderer().begin_scene(proer::graphics::color::Color { r: 0, g: 20, b: 0, a: 0 }, size);
        let layout: [VertexAtribute; 1] = [VertexAtribute { name: String::from("Pos"), datatype: VertexAttributeType::Float2, interpolate: true }];
        
        let mut vertices = [Vertex ([ 1.0, 1.0 ]), Vertex ([ 1.0, -1.0 ]), Vertex ([ -1.0, -1.0 ]), Vertex ([ -1.0, 1.0 ])];
        let mut indices = [0, 1, 2, 2, 3, 0];
        
        self.shader.as_mut().unwrap().set_uniform_float2(0, self.player_pos);
        self.shader.as_mut().unwrap().set_uniform_float(1, self.player_angle);
        self.shader.as_mut().unwrap().set_uniform_float(2, self.fov * size.0 as f32 / size.1 as f32);
        app.renderer().draw(&vertices, &indices, &layout, self.shader.as_mut().unwrap(), &self.textures[..]);

        let walk_speed = 1.0;
        let rotate_speed = 1.0;

        if self.key_states[0] {
            self.player_pos.x += self.player_angle.sin() * elapsed.as_secs_f32() * walk_speed;
            self.player_pos.y += self.player_angle.cos() * elapsed.as_secs_f32() * walk_speed; 
            if self.map.as_ref().unwrap().get_pixel(self.player_pos.x as u32, self.player_pos.y as u32).0[0] > 100 {
                self.player_pos.x -= self.player_angle.sin() * elapsed.as_secs_f32() * walk_speed;
                self.player_pos.y -= self.player_angle.cos() * elapsed.as_secs_f32() * walk_speed; 
            }
        }
        if self.key_states[1] {
            self.player_pos.x -= self.player_angle.sin() * elapsed.as_secs_f32() * walk_speed;
            self.player_pos.y -= self.player_angle.cos() * elapsed.as_secs_f32() * walk_speed; 
            if self.map.as_ref().unwrap().get_pixel(self.player_pos.x as u32, self.player_pos.y as u32).0[0] > 100 {
                self.player_pos.x += self.player_angle.sin() * elapsed.as_secs_f32() * walk_speed;
                self.player_pos.y += self.player_angle.cos() * elapsed.as_secs_f32() * walk_speed; 
            }
        }
        if self.key_states[2] {
            self.player_angle -= elapsed.as_secs_f32() * rotate_speed; }
        if self.key_states[3] {
            self.player_angle += elapsed.as_secs_f32() * rotate_speed; }

        app.renderer().end_scene();
    }

    fn on_event(&mut self, e: Event, _app: &mut Application<WindowImpl, RendererImpl>) -> bool {
        match e {
            Event::Key(Key::W, action) => { self.key_states[0] = action != Action::Release; }
            Event::Key(Key::S, action) => { self.key_states[1] = action != Action::Release; }
            Event::Key(Key::A, action) => { self.key_states[2] = action != Action::Release; }
            Event::Key(Key::D, action) => { self.key_states[3] = action != Action::Release; }
            Event::Key(Key::Escape, Action::Release) => { app.; }
            _ => {}
        }
        
        false
    }
}

fn main() {
    env_logger::init();

    Application::<proer::system::glfw::window::Window, proer::graphics::opengl::renderer::Renderer<proer::system::glfw::window::Window>>::new("proer", (800, 600), vec!(Box::new(MazeLayer::<proer::system::glfw::window::Window, proer::graphics::opengl::renderer::Renderer<proer::system::glfw::window::Window>> {player_pos: Vector2::<f32>::new(6.0, 6.0), player_angle: 0.0, shader: None, textures: Vec::new(), key_states: [false; 4], map: None, fov: 0.69})));
}