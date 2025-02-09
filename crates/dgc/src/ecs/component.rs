use std::any::Any;

pub trait Component: Any + Send + Sync {}

impl<T: Any + Send + Sync> Component for T {}

pub trait Sparse: Component {}

pub struct ComponentMgr {}

impl ComponentMgr {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ComponentMgr {
    fn default() -> Self {
        todo!()
    }
}
