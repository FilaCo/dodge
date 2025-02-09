use crate::ecs::Entity;

#[derive(Debug)]
pub struct TokenCooked;

#[derive(Debug)]
pub struct LexingRequested;

#[derive(Debug)]
pub struct LexingFinished;

#[derive(Debug)]
pub struct TokenRequested;

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    len: u32,
    extra: Option<Entity>,
}

#[derive(Debug)]
pub enum TokenKind {
    /// A line comment, e.g. `// comment`.
    LineComment,

    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment,

    /// An identifier or keyword, e.g. `ident` or `system`.
    Ident,

    /// Numerical literal, e.g. `42` or `123.321`
    NumLit,
    /// String literal, e.g. `"some string"`
    StrLit,

    /// An operator token, e.g. `>` or `|`
    Op,

    /// A punctuation token, e.g. `}` or `.`
    Punc,

    /// End of input
    Eof,
}
