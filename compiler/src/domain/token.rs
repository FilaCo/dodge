/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

/// Enum representing common lexeme types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// End of input
    EndOfFile,

    /// A line comment, e.g. `// comment`.
    LineComment,
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment,

    /// A raw identifier, embraced with backticks, e.g. `component`
    RawIdentifier,
    /// An identifier or a keyword, e.g. `component`, `Health`
    Identifier,

    Literal,

    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `/`
    Slash,
    /// `.`
    Dot,

    /// `{`
    OpenBrace,
    /// `}`
    CloseBrace,
    /// `[`
    OpenBracket,
    /// `]`
    CloseBracket,
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `;`
    Semi,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `&`
    And,
    /// `|`
    Or,
    /// `^`
    Caret,

    /// Any whitespace character sequence
    Whitespace,

    /// Any invalid or unexpected token
    Invalid,
}
