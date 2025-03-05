use crate::prelude::{Source, Spanned, TokenKind, Tokens, TrackedSpan};
use logos::Logos;
use salsa::Database;

#[salsa::tracked]
pub fn lex(db: &dyn Database, src: Source) -> Tokens<'_> {
    let content = src.content(db);

    Tokens::new(
        db,
        TokenKind::lexer(content)
            .spanned()
            .map(|(kind, span)| match kind {
                Ok(kind) => Spanned::new(kind, TrackedSpan::new(db, span.into())),
                Err(()) => Spanned::new(TokenKind::Error, TrackedSpan::new(db, span.into())),
            })
            .collect(),
    )
}
