use std::path::PathBuf;

#[salsa::input]
pub struct Source {
    #[return_ref]
    pub path: PathBuf,
    #[return_ref]
    pub content: String,
}
