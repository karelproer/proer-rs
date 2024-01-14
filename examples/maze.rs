extern crate proer;
use proer::system::window::{Window, CursorMode};
use proer::system::event::{Event, Key, Action};
use proer::graphics::renderer::Renderer;
use proer::graphics::shader::Shader;
use proer::graphics::texture::*;
use proer::graphics::vertexlayout::VertexAtribute;
use proer::graphics::vertexlayout::VertexAttributeType;
use proer::core::application::Application;
use proer::core::layer::Layer;
use std::f32;
use proer::graphics::renderable::Renderable;

use std::{boxed::Box, vec::Vec};
use nalgebra as na;
use na::{Vector2};

struct MazeLayer<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> {
    player_pos: Vector2<f32>,
    player_angle: f32,
    shader: Option<RendererImpl::ShaderType>,
    textures: Vec<RendererImpl::TextureType>,
    map: Option<image::RgbaImage>,
    key_states: [bool; 4],
    fov: f32,
    player_speed: f32,
    rectangle: Option<RendererImpl::RenderableType>,
}

#[repr(C)]
struct Vertex([f32; 2]);

impl<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> MazeLayer<WindowImpl, RendererImpl> {
    fn move_player(&mut self, dir: Vector2<f32>, elapsed: std::time::Duration) {
        const MIN_DISTANCE: f32 = 0.5;
        let new_pos_x = self.player_pos + MIN_DISTANCE * Vector2::<f32>::new(dir.x, 0.0);
        if self.map.as_ref().unwrap().get_pixel(new_pos_x.x.floor().rem_euclid(self.map.as_ref().unwrap().width() as f32) as u32, new_pos_x.y.floor().rem_euclid(self.map.as_ref().unwrap().height() as f32) as u32).0[0] < 100 {
            self.player_pos.x += dir.x * elapsed.as_secs_f32() * self.player_speed;
        }
        let new_pos_y = self.player_pos + MIN_DISTANCE * Vector2::<f32>::new(0.0, dir.y);
        if self.map.as_ref().unwrap().get_pixel(new_pos_y.x.floor().rem_euclid(self.map.as_ref().unwrap().width() as f32) as u32, new_pos_y.y.floor().rem_euclid(self.map.as_ref().unwrap().height() as f32) as u32).0[0] < 100 {
            self.player_pos.y += dir.y * elapsed.as_secs_f32() * self.player_speed;
        }
    }
}

impl<WindowImpl: Window, RendererImpl: Renderer<WindowImpl>> Layer<WindowImpl, RendererImpl> for MazeLayer<WindowImpl, RendererImpl> {
    fn on_create(&mut self, app: &mut Application<WindowImpl, RendererImpl>) {
        app.window().set_cursor_mode(CursorMode::Disabled);
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
                vec2 rayDirection = vec2(sin(rayAngle), cos(rayAngle));
                bool hitWall = false;
                float distance = 0.0;
                vec2 hitPos;
                while(!hitWall && distance <= maxDepth) {
                    distance += 0.001;

                    vec2 test = u_PlayerPos + rayDirection * distance;
                    if(textureLod(u_Map, test / textureSize(u_Map, 0), 0).x > 0.5) {
                        hitWall = true;
                        hitPos = u_PlayerPos + rayDirection * (distance + 0.001);
                    }
                }

                float height = 1.0 / (distance * cos(atan(v_Pos.x * 0.5) * u_Fov)) * 0.8;

                vec4 realColor;
                float fogFactor;

                if(v_Pos.y <= height && v_Pos.y >= -height)
                {
                    fogFactor = distance / maxDepth * 0.8;
                    vec2 hitInWall = hitPos - vec2(floor(hitPos.x) + 0.5, floor(hitPos.y) + 0.5);
                    float wallTextureX = 100;
                    
                    float PI = 3.1415926;
                    float angleInBlock = atan(hitInWall.x, hitInWall.y);

                    vec4 otherColor;
                    if(angleInBlock > 0.25 * PI && angleInBlock < 0.75 * PI) {
                        wallTextureX = hitInWall.y;
                        otherColor = vec4(1.2, 1.2, 1.1, 1.0);
                    }
                    else if(angleInBlock <= 0.25 * PI && angleInBlock > -0.25 * PI) {
                        wallTextureX = 1-hitInWall.x;
                        otherColor = vec4(1.1, 1.1, 1.05, 1.0);
                    }
                    else if(angleInBlock <= -0.25 * PI && angleInBlock > -0.75 * PI) {
                        wallTextureX = 1-hitInWall.y;
                        otherColor = vec4(1.0, 1.0, 1.0, 1.0);
                    }
                    else {
                        wallTextureX = hitInWall.x;
                        otherColor = vec4(1.1, 1.1, 1.05, 1.0);
                    }
                    realColor = otherColor * texture(u_Wall, vec2(wallTextureX, map(v_Pos.y, -height, height, 0.0, 1.0)));
                }
                else if(v_Pos.y < 0)
                {
                    fogFactor = v_Pos.y*0.6+0.6;
                    realColor = vec4(0.2, 0.4, 0.0, 1.0);
                }
                else
                {
                    fogFactor = 0;
                    realColor = vec4(0.0, 0.5, 1.0, 1.0);
                }
                color = mix(realColor, vec4(0.5, 0.5, 0.5, 1.0), fogFactor);
            }
        "#);

        let image = image::open("examples/maze.png").unwrap().into_rgba8();
        self.textures.push(RendererImpl::TextureType::new(image.clone(), SamplingMode::Nearset));
        self.map = Some(image);
        let wallimage = image::open("examples/wall.jpg").unwrap().into_rgba8();
        self.textures.push(RendererImpl::TextureType::new(wallimage.clone(), SamplingMode::Linear));
        let vertices = [Vertex ([ 1.0, 1.0 ]), Vertex ([ 1.0, -1.0 ]), Vertex ([ -1.0, -1.0 ]), Vertex ([ -1.0, 1.0 ])];
        let indices = [0, 1, 2, 2, 3, 0];
        let layout: [VertexAtribute; 1] = [VertexAtribute { name: String::from("Pos"), datatype: VertexAttributeType::Float2, interpolate: true }];
        self.rectangle = Some(RendererImpl::RenderableType::new(&vertices, &indices, &layout));
    }


    fn on_update(&mut self, elapsed: std::time::Duration, app: &mut Application<WindowImpl, RendererImpl>) {
        let size = app.get_size();
        app.renderer().begin_scene(proer::graphics::color::Color { r: 0, g: 20, b: 0, a: 0 }, size);
        
        
        let rotate_sensitivity = 0.001;
        self.player_angle = (app.get_cursor_pos().0 as f32 * rotate_sensitivity) % f32::consts::PI * 2.0;
        self.shader.as_mut().unwrap().set_uniform_float2(0, self.player_pos);
        self.shader.as_mut().unwrap().set_uniform_float(1, self.player_angle);
        self.shader.as_mut().unwrap().set_uniform_float(2, self.fov * size.0 as f32 / size.1 as f32);
        app.renderer().draw_renderable(self.rectangle.as_mut().unwrap(), self.shader.as_mut().unwrap(), &self.textures[..]);

        if self.key_states[0] {
            self.move_player(Vector2::<f32>::new(self.player_angle.sin(), self.player_angle.cos()), elapsed);
        }
        if self.key_states[1] {
            self.move_player(Vector2::<f32>::new(-self.player_angle.sin(), -self.player_angle.cos()), elapsed);
        }
        if self.key_states[2] {
            self.move_player(Vector2::<f32>::new(-self.player_angle.cos(), self.player_angle.sin()), elapsed);
        }
        if self.key_states[3] {
            self.move_player(Vector2::<f32>::new(self.player_angle.cos(), -self.player_angle.sin()), elapsed);
        }
        

        app.renderer().end_scene();
    }

    fn on_event(&mut self, e: Event, app: &mut Application<WindowImpl, RendererImpl>) -> bool {
        match e {
            Event::Key(Key::W, action) => { self.key_states[0] = action != Action::Release; }
            Event::Key(Key::S, action) => { self.key_states[1] = action != Action::Release; }
            Event::Key(Key::A, action) => { self.key_states[2] = action != Action::Release; }
            Event::Key(Key::D, action) => { self.key_states[3] = action != Action::Release; }
            Event::Key(Key::Escape, Action::Release) => { app.window().set_cursor_mode(CursorMode::Normal); }
            Event::Key(Key::Space, Action::Release)  => { app.window().set_cursor_mode(CursorMode::Disabled); }
            _ => {}
        }
        
        false
    }
}

fn main() {
    env_logger::init();

    Application::<proer::system::glfw::window::Window, proer::graphics::opengl::renderer::Renderer<proer::system::glfw::window::Window>>::new("proer", (800, 600), vec!(Box::new(MazeLayer::<proer::system::glfw::window::Window, proer::graphics::opengl::renderer::Renderer<proer::system::glfw::window::Window>> {player_pos: Vector2::<f32>::new(6.5, 6.5), player_angle: 0.0, shader: None, textures: Vec::new(), key_states: [false; 4], map: None, fov: 0.69, player_speed: 5.0, rectangle: None})));
}