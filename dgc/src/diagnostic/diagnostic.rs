use crate::diagnostic::{Span, Spanned};
use crate::query::LexError;
use ariadne::ReportKind;
use std::fmt::Debug;

#[salsa::accumulator]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub msg: String,
    pub span: Span,
    pub labels: Vec<Label>,
}

impl Diagnostic {
    pub fn new(
        kind: DiagnosticKind,
        msg: impl Into<String>,
        span: Span,
        labels: Vec<Label>,
    ) -> Self {
        Self {
            kind,
            msg: msg.into(),
            span,
            labels,
        }
    }
}

impl From<Spanned<LexError>> for Diagnostic {
    fn from(value: Spanned<LexError>) -> Self {
        Self::new(
            DiagnosticKind::Error,
            format!("{}", value.value),
            value.span,
            vec![],
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Advice,
}

impl From<ReportKind<'_>> for DiagnosticKind {
    fn from(value: ReportKind) -> Self {
        match value {
            ReportKind::Error => Self::Error,
            ReportKind::Warning => Self::Warning,
            _ => Self::Advice,
        }
    }
}

impl From<DiagnosticKind> for ReportKind<'_> {
    fn from(value: DiagnosticKind) -> Self {
        match value {
            DiagnosticKind::Error => Self::Error,
            DiagnosticKind::Warning => Self::Warning,
            DiagnosticKind::Advice => Self::Advice,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub span: Span,
    pub msg: Option<String>,
}
