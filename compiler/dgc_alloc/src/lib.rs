// TODO: Remove this macros when project is mature enough
// < immature_allowances
#![allow(dead_code)]
#![allow(unused)]
// > immature_allowances
#![feature(allocator_api)]
#![feature(sized_type_properties)]

mod linear;

pub mod prelude {
    pub use crate::linear::*;
}
