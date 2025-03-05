use salsa::{Database, Event, Storage};

#[salsa::db]
#[derive(Clone, Default)]
pub struct DodgeCompilerDatabase {
    storage: Storage<Self>,
}

#[salsa::db]
impl Database for DodgeCompilerDatabase {
    fn salsa_event(&self, _: &dyn Fn() -> Event) {}
}
