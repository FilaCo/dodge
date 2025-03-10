use crate::ir::{NumVal, StrVal, Symbol};

#[derive(Debug)]
pub enum Token<'db> {
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment,

    /// A line comment, e.g. `// comment`.
    LineComment,

    /// Any whitespace character sequence.
    Whitespace,

    /// An identifier or keyword, e.g. `Foo` or `component`.
    Ident(Symbol<'db>),

    /// Literals, e.g. `"some string"`, `42`, `1e8`, `123.321`, `true`.
    Lit(LitKind<'db>),

    /// `;`
    Semi,
    /// `,`
    Comma,
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
    /// `@`
    At,
    /// `#`
    Hash,
    /// `~`
    Tilde,
    /// `?`
    Question,
    /// `:`
    Colon,
    /// `$`
    Dollar,
    /// `=`
    Eq,
    /// `!`
    Exclamation,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `-`
    Minus,
    /// `&`
    Amp,
    /// `|`
    Pipe,
    /// `+`
    Plus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `^`
    Caret,
    /// `%`
    Percent,
}

#[derive(salsa::Update, Debug)]
pub enum LitKind<'db> {
    Bool(bool),
    Num(NumVal<'db>),
    Str(StrVal<'db>),
}
