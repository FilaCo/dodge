use crate::prelude::{parse, Source};
use salsa::Database;

#[salsa::tracked]
pub fn compile(db: &dyn Database, src: Source) {
    parse(db, src);
}
