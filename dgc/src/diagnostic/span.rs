use chumsky::prelude::SimpleSpan;

pub type Span = SimpleSpan;

#[derive(salsa::Update, Debug)]
pub struct Spanned<T: salsa::Update> {
    pub value: T,
    pub span: Span,
}

impl<T: salsa::Update> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}
