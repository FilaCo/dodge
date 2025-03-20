use dgc_alloc::prelude::Linear;
use std::alloc::Global;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Entity(usize);

impl Entity {
    fn position(&self) -> usize {
        self.0
    }

    fn generation(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct EntityManager {
    entities: Vec<Entity, Linear<'static, Global>>,
    available: AtomicUsize,
    next_pos: AtomicUsize,
}

impl EntityManager {
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    pub fn spawn(&mut self) -> Entity {
        if self.available.load(Ordering::SeqCst) > 0 {
            // let holder = self.entities[self.next_pos];
            todo!()
        } else {
            let entity = Entity(self.next_pos.fetch_add(1, Ordering::SeqCst));
            self.entities.push(entity);

            entity
        }
    }

    pub fn despawn(&mut self, entity: Entity) {
        todo!()
    }

    pub fn iter(&self) -> EntityIter<'_> {
        todo!()
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a EntityManager {
    type Item = Entity;
    type IntoIter = EntityIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct EntityIter<'a> {
    _m: PhantomData<&'a Entity>,
}

impl Iterator for EntityIter<'_> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
