use crate::ecs::{ComponentMgr, EntityMgr, SystemMgr};
use crate::util::{EventBus, ThreadPool};
use crate::DGCEvent;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub struct World {
    id: WorldId,
    name: WorldName,
    em: EntityMgr,
    cm: ComponentMgr,
    sm: SystemMgr,

    eb: Arc<EventBus>,
    tp: Arc<ThreadPool>,
}

impl World {
    pub fn new(id: WorldId, name: WorldName) -> Self {
        todo!()
    }

    pub fn main_loop(&self) {
        loop {
            self.sm.before_update();

            while self.need_fixed_update() {
                self.sm.fixed_update()
            }

            self.sm.update();
            self.sm.after_update();
        }
    }

    pub fn query<Component, Filter>(&self) {
        todo!()
    }

    fn need_fixed_update(&self) -> bool {
        todo!()
    }
}

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for World {}

impl Hash for World {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WorldId(u64);

impl From<u64> for WorldId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorldName(String);

impl From<String> for WorldName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for WorldName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
