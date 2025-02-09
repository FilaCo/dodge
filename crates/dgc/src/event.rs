#[derive(Debug, Clone)]
pub struct DGCEvent {
    pub kind: DGCEventKind,
}

impl DGCEvent {
    pub fn new(kind: DGCEventKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, Clone)]
pub enum DGCEventKind {
    CompilationRequested,
    CompilationFailed,
    CompilationFinished,
}
