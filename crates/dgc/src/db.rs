use salsa::{Database, Storage};

#[salsa::database]
pub struct DodgeDb {
    storage: Storage<Self>,
}

impl Database for DodgeDb {}
