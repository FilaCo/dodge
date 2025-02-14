use std::path::Path;
use std::sync::Arc;

#[salsa::input]
pub struct Source {
    pub path: Arc<Path>,
    #[return_ref]
    pub contents: String,
}

#[salsa::tracked]
pub struct Ast<'db> {}
