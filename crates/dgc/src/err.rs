use std::sync::mpsc::RecvError;

/// A type that represents a common DGC operation result.
pub type DGCResult<T> = Result<T, DGCErr>;

/// A DGC operation error type.
///
/// See [DGCErrKind] for more details.
#[derive(Debug)]
pub struct DGCErr {
    pub kind: DGCErrKind,
    pub msg: Option<String>,
    pub cause: Option<Box<DGCErr>>,
}

impl DGCErr {
    pub fn new(kind: DGCErrKind, msg: Option<String>, cause: Option<Box<DGCErr>>) -> Self {
        Self { kind, msg, cause }
    }
}

/// An enum of all possible DGC operation error kinds.
#[derive(Debug)]
pub enum DGCErrKind {
    CompilationFailed,
}

impl From<RecvError> for DGCErr {
    fn from(value: RecvError) -> Self {
        todo!()
    }
}
