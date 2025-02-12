pub type Entity = usize;

pub struct EntityMgr {
    entities: Vec<Entity>,
    available: usize,
    next_pos: usize,
}

impl EntityMgr {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            available: 0,
            next_pos: 0,
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let mut entity = self.next_pos;

        if self.available > 0 {
            entity |= entity_gen(self.entities[self.next_pos]);

            self.available -= 1;
            self.next_pos = entity_pos(self.entities[self.next_pos]);
        } else {
            self.entities.push(entity);
            self.next_pos += 1;

            assert!(self.next_pos <= entity_part_max_val());
        }

        entity
    }

    pub fn despawn(&mut self, entity: Entity) -> &mut Self {
        let entity_pos = entity_pos(entity);
        let entity_gen = entity_gen(entity) + 1;

        assert!(entity_gen <= entity_part_max_val());
        assert!(self.entities.len() > entity_pos);

        self.entities[entity_pos] = self.next_pos | shift_entity_gen(entity_gen);

        self.next_pos = entity_pos;
        self.available += 1;

        self
    }
}

impl Default for EntityMgr {
    fn default() -> Self {
        Self::new()
    }
}

fn entity_pos(entity: Entity) -> usize {
    entity & entity_pos_mask()
}

fn entity_pos_mask() -> usize {
    match size_of::<usize>() {
        8 => 0xFFFFFFFF,
        4 => 0xFFFF,
        2 => 0xFF,
        _ => panic!("unexpected usize size"),
    }
}

fn entity_gen(entity: Entity) -> usize {
    entity >> (usize::BITS >> 1)
}

fn shift_entity_gen(gen: usize) -> usize {
    gen << (usize::BITS >> 1)
}

fn entity_part_max_val() -> usize {
    usize::MAX >> (usize::BITS >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_spawn_works() {
        // arrange
        let mut em = EntityMgr::default();
        let expected: Entity = 0;

        // act
        let actual = em.spawn();

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn entity_despawn_works() {
        // arrange
        let mut em = EntityMgr::default();
        let entity = em.spawn();

        let expected_available = 1;
        let expected_entity = 1 | shift_entity_gen(1);

        // act
        em.despawn(entity);

        // assert
        assert_eq!(expected_available, em.available);
        assert_eq!(entity_pos(entity), em.next_pos);
        assert_eq!(expected_entity, em.entities[entity]);
    }

    #[test]
    fn entity_recycling_works() {
        // arrange
        let mut em = EntityMgr::default();

        let expected_entities_len = 10;

        // act
        for _ in 0..10 {
            em.spawn();
        }

        for ent in (0..10).step_by(2) {
            em.despawn(ent as Entity);
        }

        for _ in 0..5 {
            em.spawn();
        }

        // assert
        assert_eq!(expected_entities_len, em.entities.len());
    }
}
