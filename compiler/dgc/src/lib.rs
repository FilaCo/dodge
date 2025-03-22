// TODO: Remove this macros when project is mature enough
// < immature_allowances
#![allow(dead_code)]
#![allow(unused)]
// > immature_allowances

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL_ALLOC: Jemalloc = Jemalloc;

mod app;
mod feature;

pub mod prelude {
    pub use crate::app::*;
}
