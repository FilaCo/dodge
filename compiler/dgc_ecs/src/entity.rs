use dgc_alloc::prelude::Linear;
use std::alloc::Global;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Entity(usize);

impl Entity {
    #[inline(always)]
    const fn new(position: usize, generation: usize) -> Self {
        Entity(position | Self::shl_generation(generation))
    }

    #[inline(always)]
    const fn position(&self) -> usize {
        self.0 & Self::position_mask()
    }

    #[inline(always)]
    const fn generation(&self) -> usize {
        Self::shr_generation(self.0)
    }

    #[inline(always)]
    const fn shr_generation(generation: usize) -> usize {
        generation >> Self::position_bits()
    }

    #[inline(always)]
    const fn shl_generation(generation: usize) -> usize {
        generation << Self::position_bits()
    }

    #[inline(always)]
    const fn position_mask() -> usize {
        (1 << Self::position_bits()) - 1
    }

    #[inline(always)]
    const fn position_bits() -> usize {
        size_of::<usize>() * 5
    }
}

#[derive(Debug)]
pub struct EntityManager {
    entities: Vec<Entity, Linear<'static, Global>>,
    available: usize,
    next_position: usize,
}

impl EntityManager {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entities: Vec::with_capacity_in(
                capacity,
                Linear::with_capacity_in(capacity * size_of::<Entity>(), &Global),
            ),
            available: 0,
            next_position: 0,
        }
    }

    pub fn spawn(&mut self) -> Entity {
        if self.available > 0 {
            self.recycle()
        } else {
            self.spawn_impl()
        }
    }

    pub fn despawn(&mut self, entity: Entity) {
        // holder stores previous next_position and actual generation of the entity
        let holder = Entity::new(self.next_position, entity.generation() + 1);
        self.entities[entity.position()] = holder;

        // restore invariants
        self.next_position = entity.position();
        self.available += 1;
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        entity.generation() == self.entities[entity.position()].generation()
    }

    pub fn is_dead(&self, entity: Entity) -> bool {
        !self.is_alive(entity)
    }

    #[inline(always)]
    fn recycle(&mut self) -> Entity {
        // holder stores recycled entity generation
        let holder = self.entities[self.next_position];

        let recycled = Entity::new(self.next_position, holder.generation());

        // restore invariants
        self.next_position = holder.position();
        self.available -= 1;

        recycled
    }

    #[inline(always)]
    fn spawn_impl(&mut self) -> Entity {
        // new spawned entity has version equals to 0
        let spawned = Entity(self.next_position);
        self.entities.push(spawned);

        // restore invariants
        self.next_position += 1;

        spawned
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_spawns_entity() {
        // arrange
        let mut em = EntityManager::default();
        let expected: Entity = Entity(0);

        // act
        let actual = em.spawn();

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_despawns_entity() {
        // arrange
        let mut em = EntityManager::default();
        let entity = em.spawn();

        let expected_available = 1;
        let expected_entity = Entity::new(1, 1);

        // act
        em.despawn(entity);

        // assert
        assert_eq!(expected_available, em.available);
        assert_eq!(entity.position(), em.next_position);
        assert_eq!(expected_entity, em.entities[entity.position()]);
    }

    #[test]
    fn it_recycle_entities() {
        // arrange
        let mut em = EntityManager::default();

        let expected_entities_len = 10;

        // act
        for _ in 0..10 {
            em.spawn();
        }

        for ent in (0..10).step_by(2) {
            em.despawn(Entity(ent));
        }

        for _ in 0..5 {
            em.spawn();
        }

        // assert
        assert_eq!(expected_entities_len, em.entities.len());
    }
}
