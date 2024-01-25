extern crate legion;
use legion::{World, Schedule, Resources};
use super::entity::Entity;

pub use legion::systems::Resource;

pub struct Scene {
    pub world: World,
    pub resources: Resources,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: World::default(),
            resources: Resources::default(),
        }
    }

    pub fn create_entity<T: legion::storage::Component>(&mut self, components: T) -> Entity 
    where Option<T>: legion::storage::IntoComponentSource {
        self.world.push(components)
    }

    pub fn run(&mut self, schedule: &mut Schedule) {
        schedule.execute(&mut self.world, &mut self.resources);
    }
}