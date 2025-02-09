use crate::ecs::World;

pub trait System {
    fn before_update(&self) {}
    fn fixed_update(&self) {}
    fn update(&self, w: &World) {}
    fn after_update(&self) {}
}

pub struct SystemMgr {}

impl SystemMgr {
    pub fn new() -> Self {
        Self {}
    }

    pub fn before_update(&self) {
        todo!()
    }

    pub fn fixed_update(&self) {
        todo!()
    }

    pub fn update(&self) {
        todo!()
    }

    pub fn after_update(&self) {
        todo!()
    }
}

impl Default for SystemMgr {
    fn default() -> Self {
        todo!()
    }
}
