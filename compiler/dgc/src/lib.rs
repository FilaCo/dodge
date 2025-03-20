// TODO: Remove this macros when project is mature enough
// < immature_allowances
#![allow(dead_code)]
#![allow(unused)]
// > immature_allowances

mod app;

pub mod prelude {
    pub use crate::app::*;
}
