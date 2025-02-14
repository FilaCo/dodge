use crate::prelude::Source;
use salsa::Database;

#[salsa::tracked]
pub fn compile(db: &dyn Database, src: Source) {}
