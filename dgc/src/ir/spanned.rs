use chumsky::prelude::SimpleSpan;

pub type Span = SimpleSpan;

pub struct Spanned<T> {
    pub val: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(val: T, span: Span) -> Self {
        Self { val, span }
    }
}
