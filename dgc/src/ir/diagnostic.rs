use crate::prelude::Span;
use ariadne::ReportKind;
use std::fmt::Debug;

#[salsa::accumulator]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    pub span: Span,
    pub labels: Vec<Label>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Advice,
}

impl From<ReportKind<'static>> for DiagnosticKind {
    fn from(value: ReportKind) -> Self {
        match value {
            ReportKind::Error => Self::Error,
            ReportKind::Warning => Self::Warning,
            _ => Self::Advice,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub span: Span,
    pub msg: Option<String>,
}
