use crate::domain::Entity;

#[derive(Debug)]
pub struct LexingStarted;

#[derive(Debug)]
pub struct LexingFinished;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    kind: Entity,
    len: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenKind {
    /// End of input.
    Eof,
    /// Any whitespace character sequence.
    Whitespace,
    /// A line (e.g. `// line comment`) or block comment (e.g. `/* block comment */`).
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    Comment,
    /// Literals, e.g. `42`, "Hello, world!"
    ///
    /// See [LitKind] for more details
    Lit,
    /// Identifiers or keywords e.g. `ident` or `component`
    Ident,
    /// Operator
    Op,
    Punc,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum OpKind {
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    Lt,
    Gt,
    And,
    Or,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PuncKind {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Colon,
    Semi,
    Dot,
    Comma,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LitKind {
    Number,
    String,
}
