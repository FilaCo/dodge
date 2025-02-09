use crate::ecs::{World, WorldId, WorldName};
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

pub struct WorldMgr {
    last: AtomicU64,
    worlds: Arc<Mutex<HashSet<World>>>,
}

impl WorldMgr {
    pub fn new() -> Self {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        self.worlds
            .lock()
            .expect("unable to acquire lock")
            .is_empty()
    }

    pub fn spawn(&self, name: WorldName) -> WorldId {
        let id: WorldId = self.last.fetch_add(1, Ordering::Relaxed).into();
        let world = World::new(id, name);

        self.worlds
            .lock()
            .expect("unable to acquire lock")
            .insert(world);

        id
    }

    pub fn iter(&self) -> Iter {
        todo!()
    }
}

impl Default for WorldMgr {
    fn default() -> Self {
        todo!()
    }
}

pub struct Iter {}

impl Iterator for Iter {
    type Item = World;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
