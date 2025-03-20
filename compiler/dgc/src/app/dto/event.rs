#[derive(Debug)]
#[non_exhaustive]
pub struct DgcEvent {
    pub kind: DgcEventKind,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DgcEventKind {
    CompilationRequested,
    CompilationStarted,
    CompilationFailed,
    CompilationFinished,
}
