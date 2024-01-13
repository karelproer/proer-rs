extern crate legion;
use legion::{World, Schedule, Resources};
use super::entity::Entity;

pub type Resource = legion::Resource;

pub struct Scene {
    world: World,
    recources: Resources,
}

impl Scene {
    fn new() {
        Self {
            World::Defualt(),
            Resources::::Defualt();
        }
    }

    fn create_entity<T>(&mut self, components: T) -> Entity {
        world.push(components)
    }

    fn execute(&mut self, schelude: Schedule) {
        schedule.execute(&mut self.world, &mut self.resources);
    }
}