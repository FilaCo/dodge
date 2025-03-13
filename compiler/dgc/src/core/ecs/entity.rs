use crate::alloc::Linear;
use crate::core::collection::{ArrayList, List};
use core::ptr::NonNull;
use core::sync::atomic::AtomicUsize;
use core_alloc::boxed::Box;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(usize);

#[derive(Debug)]
pub struct EntityManager {
    entities: NonNull<dyn List<Entity>>,
    next_idx: AtomicUsize,
    available: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        let boxed = Box::new(ArrayList::<Entity, Linear>::new());

        Self {
            entities: NonNull::new(Box::into_raw(boxed)).unwrap(),
            next_idx: AtomicUsize::new(0),
            available: 0,
        }
    }

    pub fn spawn(&mut self) -> Entity {
        if self.available > 0 {
            self.recycle()
        } else {
            self.spawn_impl()
        }
    }

    pub fn despawn(&mut self, entity: Entity) -> &mut Self {
        self
    }

    fn recycle(&mut self) -> Entity {
        todo!()
    }

    fn spawn_impl(&mut self) -> Entity {
        todo!()
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}
