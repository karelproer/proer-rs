use crate::graphics::renderer::Renderer;
use super::super::scene::Scene;
use super::super::components::{spriterenderer::SpriteRenderer, transform::Transform, camera::Camera};
use std::{rc::Rc, cell::RefCell, sync::{Arc, Mutex}};
use legion::{Query, world::SubWorld, system};
    use nalgebra::{Matrix4, Orthographic3};

pub trait SceneRenderer {
    fn render(&mut self, spriterenderer: &SpriteRenderer, transform: &Transform);
}

#[system]
pub fn render_scene(#[state] renderer: &Arc<Mutex<dyn SceneRenderer>>, world: &mut SubWorld, cameras: &mut Query<(&Transform, &Camera)>, sprites: &mut Query<(&Transform, &SpriteRenderer)>)
{
    let mut viewproj: Matrix4<f32> = *Orthographic3::<f32>::from_fov(1.0, 90.0, 0.1, 1000.0).as_matrix();
    for (transform, camera) in cameras.iter(world) {
        if camera.active {
            viewproj = Orthographic3::<f32>::from_fov(1.0, camera.fov, camera.near, camera.far).as_matrix() * transform.transform.to_homogeneous();
        }
    }
    
    for (transform, sprite) in sprites.iter(world) {
        renderer.lock().unwrap().render(sprite, transform);
    }
}