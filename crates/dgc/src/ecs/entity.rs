#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(u64);

impl Entity {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EntityMgr {
    last: Entity,
}

impl EntityMgr {
    pub fn new() -> Self {
        todo!()
    }

    pub fn spawn<Component>(&mut self, component: Component) -> EntityBuilder {
        todo!()
    }

    pub fn despawn(&mut self, entity: Entity) -> &mut Self {
        todo!()
    }
}

impl Default for EntityMgr {
    fn default() -> Self {
        todo!()
    }
}

pub struct EntityBuilder {
    id: Entity,
}

impl EntityBuilder {
    pub fn new(id: Entity) -> Self {
        Self { id }
    }

    pub fn insert<Component>(&mut self, component: Component) -> &mut Self {
        todo!()
    }
}
