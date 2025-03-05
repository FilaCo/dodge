use salsa::{Database, Event, Storage};

#[salsa::db]
#[derive(Clone, Default)]
pub struct DodgeCompilerDatabase {
    storage: Storage<Self>,
}

#[salsa::db]
impl Database for DodgeCompilerDatabase {
    fn salsa_event(&self, event: &dyn Fn() -> Event) {
        // TODO: Handle events?
        let _ = event();
    }
}
