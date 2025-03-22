use crate::{
    component::ComponentPool,
    prelude::{Entity, EntitySet},
};

#[derive(Debug)]
pub struct World {
    entities: EntitySet,
    components: ComponentPool,
    systems: Vec<Box<dyn FnMut(&mut Self)>>,
}

impl World {
    pub fn spawn(&mut self) {}

    pub fn despawn(&mut self, entity: Entity) {}

    pub fn run(&mut self) {
        self.systems.iter_mut().for_each(|sys| sys(self));
    }
}

#[derive(Debug)]
pub struct WorldBuilder;

impl WorldBuilder {
    pub fn register_component<ComponentType>(mut self) {}

    pub fn register_system(mut self, system: fn(&mut Self)) {}
}
