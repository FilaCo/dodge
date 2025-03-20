use dgc_alloc::prelude::Linear;
use std::alloc::Global;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity(usize);

#[derive(Debug)]
pub struct EntityManager {
    entities: Vec<Entity, Linear<'static, Global>>,
    available: usize,
    next_pos: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    pub fn spawn(&mut self) -> Entity {
        todo!()
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
