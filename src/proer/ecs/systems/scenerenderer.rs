use super::super::components::{spriterenderer::SpriteRenderer, transform::Transform, camera::Camera};
use std::{sync::{Arc, Mutex}};
use legion::{Query, world::SubWorld, system};
    use nalgebra::{Matrix4, Orthographic3};

pub trait SceneRenderer {
    fn set_camera(&mut self, camera: &Matrix4<f32>);
    fn get_aspectratio(&self) -> f32;

    fn render(&mut self, spriterenderer: &SpriteRenderer, transform: &Transform);
}

#[system]
pub fn render_scene(#[state] renderer: &Arc<Mutex<dyn SceneRenderer>>, world: &mut SubWorld, cameras: &mut Query<(&Transform, &Camera)>, sprites: &mut Query<(&Transform, &SpriteRenderer)>)
{
    let mut r = renderer.lock().unwrap();
    for (transform, camera) in cameras.iter(world) {
        if camera.active {
            let aspectratio = r.get_aspectratio();
            r.set_camera(&(Orthographic3::<f32>::from_fov(aspectratio, camera.fov, camera.near, camera.far).as_matrix() * transform.transform.to_homogeneous()));
            break;
        }
    }
    
    for (transform, sprite) in sprites.iter(world) {
        r.render(sprite, transform);
    }
}