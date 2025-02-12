use crate::ecs::World;

pub trait System {
    fn before_update(&self) {}
    fn fixed_update(&self) {}
    fn update(&self, w: &World) {}
    fn after_update(&self) {}
}
