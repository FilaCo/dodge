// TODO: Remove this macros when project is mature enough
// < immature_allowances
#![allow(dead_code)]
#![allow(unused)]
// > immature_allowances
#![feature(allocator_api)]
mod entity;
mod world;

pub mod prelude {
    pub use crate::world::*;
}
