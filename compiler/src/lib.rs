mod db;
mod diagnostic;
mod ir;
mod query;

pub mod prelude {
    pub use crate::db::*;
    pub use crate::diagnostic::*;
    pub use crate::ir::*;
    pub use crate::query::*;
}
