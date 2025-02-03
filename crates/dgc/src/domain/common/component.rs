use crate::domain::Entity;

#[derive(Debug)]
pub struct CompilationStarted {
    pub source: Entity,
}
