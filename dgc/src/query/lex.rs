use crate::diagnostic::{Diagnostic, Spanned};
use crate::ir::{NumVal, StrVal, Symbol};
use crate::prelude::{LitKind as CookedLitKind, Source, Token as CookedToken, Tokens};
use chumsky::prelude::{just, recursive};
use chumsky::Parser;
use logos::{Lexer, Logos, Skip};
use salsa::{Accumulator, Database};
use std::collections::VecDeque;
use thiserror::Error;

#[derive(Error, salsa::Update, Debug, Default, Copy, Clone, PartialEq)]
pub enum LexError {
    #[error("unterminated block comment")]
    UnTerminatedBlockComment,
    #[default]
    #[error("internal lexer error")]
    Internal,
}

#[salsa::tracked]
pub fn lex(db: &dyn Database, src: Source) -> Tokens<'_> {
    let content = src.content(db);

    let (tokens, errs): (Vec<_>, Vec<_>) = Token::lexer(content)
        .spanned()
        .partition(|(t, _)| t.is_ok());

    errs.into_iter()
        .map(|(t, s)| (t.expect_err("errors partition"), s.into()))
        .map(|(t, s)| Spanned::new(t, s).into())
        .for_each(|diag: Diagnostic| diag.accumulate(db));

    let cooked_tokens: Vec<_> = tokens
        .into_iter()
        .map(|(t, s)| (t.expect("tokens partition"), s.into()))
        .map(|(t, s)| {
            (
                match t {
                    Token::BlockComment => CookedToken::BlockComment,
                    Token::LineComment => CookedToken::LineComment,
                    Token::Whitespace => CookedToken::Whitespace,
                    Token::Ident(ident) => CookedToken::Ident(Symbol::new(db, ident)),
                    Token::Lit(kind) => match kind {
                        LitKind::Bool(b) => CookedToken::Lit(CookedLitKind::Bool(b)),
                        LitKind::Num(n) => CookedToken::Lit(CookedLitKind::Num(NumVal::new(db, n))),
                        LitKind::Str(s) => CookedToken::Lit(CookedLitKind::Str(StrVal::new(db, s))),
                    },
                    Token::Semi => CookedToken::Semi,
                    Token::Comma => CookedToken::Comma,
                    Token::Dot => CookedToken::Dot,
                    Token::OpenBrace => CookedToken::OpenBrace,
                    Token::CloseBrace => CookedToken::CloseBrace,
                    Token::OpenBracket => CookedToken::OpenBracket,
                    Token::CloseBracket => CookedToken::CloseBracket,
                    Token::OpenParen => CookedToken::OpenParen,
                    Token::CloseParen => CookedToken::CloseParen,
                    Token::At => CookedToken::At,
                    Token::Hash => CookedToken::Hash,
                    Token::Tilde => CookedToken::Tilde,
                    Token::Question => CookedToken::Question,
                    Token::Colon => CookedToken::Colon,
                    Token::Dollar => CookedToken::Dollar,
                    Token::Eq => CookedToken::Eq,
                    Token::Exclamation => CookedToken::Exclamation,
                    Token::Lt => CookedToken::Lt,
                    Token::Gt => CookedToken::Gt,
                    Token::Minus => CookedToken::Minus,
                    Token::Amp => CookedToken::Amp,
                    Token::Pipe => CookedToken::Pipe,
                    Token::Plus => CookedToken::Plus,
                    Token::Star => CookedToken::Star,
                    Token::Slash => CookedToken::Slash,
                    Token::Caret => CookedToken::Caret,
                    Token::Percent => CookedToken::Percent,
                },
                s,
            )
        })
        .map(|(t, s)| Spanned::new(t, s))
        .collect();

    Tokens::new(db, cooked_tokens)
}

#[derive(Logos, Debug, Copy, Clone, PartialEq)]
#[logos(subpattern dec_digits = r"\d(_?\d)*")]
#[logos(subpattern bin_digits = r"[01](_?[01])*")]
#[logos(subpattern oct_digits = r"[0-7](_?[0-7])*")]
#[logos(subpattern hex_digits = r"[\da-fA-F](_?[\da-fA-F])*")]
#[logos(subpattern dec_int_lit = r"0|[1-9][_?(?&dec_digits)]?")]
#[logos(subpattern bin_int_lit = r"0[bB]_?(?&bin_digits)")]
#[logos(subpattern oct_int_lit = r"0[oO]_?(?&oct_digits)")]
#[logos(subpattern hex_int_lit = r"0[xX]_?(?&hex_digits)")]
#[logos(subpattern int_lit = r"(?&dec_int_lit)|(?&bin_int_lit)|(?&oct_int_lit)|(?&hex_int_lit)")]
#[logos(subpattern hex_exp = r"[pP][+-]?(?&dec_digits)")]
#[logos(subpattern hex_mantissa = r"(_?(?&hex_digits)\.(?&hex_digits)?)|(_?(?&hex_digits))|(\.(?&hex_digits))")]
#[logos(subpattern hex_float_lit = r"0[xX](?&hex_mantissa)(?&hex_exp)")]
#[logos(subpattern dec_exp = r"[eE][+-]?(?&dec_digits)")]
#[logos(subpattern dec_float_lit = r"((?&dec_digits)\.(?&dec_digits)?(?&dec_exp)?)|((?&dec_digits)(?&dec_exp))|(\.(?&dec_digits)(?&dec_exp)?)")]
#[logos(subpattern float_lit = r"(?&dec_float_lit)|(?&hex_float_lit)")]
#[logos(subpattern esc_char = r#"\\[abfnrtv\\'"]"#)]
#[logos(subpattern big_u_val = r"\\U[\da-fA-F]{8}")]
#[logos(subpattern little_u_val = r"\\u[\da-fA-F]{4}")]
#[logos(subpattern hex_byte_val = r"\\x[\da-fA-F]{2}")]
#[logos(subpattern oct_byte_val = r"\\[0-7]{3}")]
#[logos(subpattern byte_val = r"(?&oct_byte_val)|(?&hex_byte_val)")]
#[logos(subpattern unicode_val = r"[^\n]|(?&little_u_val)|(?&big_u_val)|(?&esc_char)")]
#[logos(subpattern bool_lit = r"true|false")]
#[logos(subpattern num_lit = r"(?&int_lit)|(?&float_lit)")]
#[logos(subpattern str_lit = r#""[(?&unicode_val)(?&byte_val)]*""#)]
#[logos(subpattern ident = r"[\p{XID_Start}_]\p{XID_Continue}*")]
#[logos(error = LexError)]
enum Token<'db> {
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    #[token("/*", block_comment)]
    BlockComment,

    /// A line comment, e.g. `// comment`.
    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,

    /// Any whitespace character sequence.
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    Whitespace,

    /// An identifier or keyword, e.g. `Foo` or `component`.
    #[regex(r"(?&ident)", ident)]
    Ident(&'db str),

    /// Literals, e.g. `"some string"`, `42`, `1e8`, `123.321`, `true`.
    #[regex(r"(?&bool_lit)", bool_lit)]
    #[token(r"(?&str_lit)", str_lit)]
    #[regex(r"(?&num_lit)", num_lit)]
    Lit(LitKind<'db>),

    /// `;`
    #[token(";")]
    Semi,
    /// `,`
    #[token(",")]
    Comma,
    /// `.`
    #[token(".")]
    Dot,
    /// `{`
    #[token("{")]
    OpenBrace,
    /// `}`
    #[token("}")]
    CloseBrace,
    /// `[`
    #[token("[")]
    OpenBracket,
    /// `]`
    #[token("]")]
    CloseBracket,
    /// `(`
    #[token("(")]
    OpenParen,
    /// `)`
    #[token(")")]
    CloseParen,
    /// `@`
    #[token("@")]
    At,
    /// `#`
    #[token("#")]
    Hash,
    /// `~`
    #[token("~")]
    Tilde,
    /// `?`
    #[token("?")]
    Question,
    /// `:`
    #[token(":")]
    Colon,
    /// `$`
    #[token("$")]
    Dollar,
    /// `=`
    #[token("=")]
    Eq,
    /// `!`
    #[token("!")]
    Exclamation,
    /// `<`
    #[token("<")]
    Lt,
    /// `>`
    #[token(">")]
    Gt,
    /// `-`
    #[token("-")]
    Minus,
    /// `&`
    #[token("&")]
    Amp,
    /// `|`
    #[token("|")]
    Pipe,
    /// `+`
    #[token("+")]
    Plus,
    /// `*`
    #[token("*")]
    Star,
    /// `/`
    #[token("/")]
    Slash,
    /// `^`
    #[token("^")]
    Caret,
    /// `%`
    #[token("%")]
    Percent,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum LitKind<'db> {
    Bool(bool),
    Num(&'db str),
    Str(&'db str),
}

fn block_comment<'db>(lex: &mut Lexer<'db, Token<'db>>) -> Result<Skip, LexError> {
    const OPEN_BLOCK_COMMENT: &str = "/*";
    const CLOSE_BLOCK_COMMENT: &str = "*/";

    let to_close = lex.remainder().find(CLOSE_BLOCK_COMMENT);
    let to_open = lex.remainder().find(OPEN_BLOCK_COMMENT);

    Err(LexError::UnTerminatedBlockComment)
}

fn ident<'db>(lex: &mut Lexer<'db, Token<'db>>) -> Result<&'db str, LexError> {
    Ok(lex.slice())
}

fn bool_lit<'db>(lex: &mut Lexer<'db, Token<'db>>) -> Result<LitKind<'db>, LexError> {
    match lex.slice() {
        "true" => Ok(LitKind::Bool(true)),
        "false" => Ok(LitKind::Bool(false)),
        _ => unreachable!("bool literals are always \"true\" or \"false\"."),
    }
}

fn str_lit<'db>(lex: &mut Lexer<'db, Token<'db>>) -> Result<LitKind<'db>, LexError> {
    Ok(LitKind::Str(lex.slice()))
}

fn num_lit<'db>(lex: &mut Lexer<'db, Token<'db>>) -> Result<LitKind<'db>, LexError> {
    Ok(LitKind::Num(lex.slice()))
}
