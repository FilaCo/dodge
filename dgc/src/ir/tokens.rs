use crate::prelude::Spanned;
use logos::Logos;

#[salsa::tracked]
pub struct Tokens<'db> {
    #[tracked]
    #[return_ref]
    pub val: Vec<Spanned<'db, TokenKind<'db>>>,
}

#[derive(Logos, salsa::Update, Default, Clone, PartialEq)]
pub enum TokenKind<'src> {
    #[regex("true|false", |lex| lex.slice().parse::<bool>().expect("unable to parse bool literal"))]
    BoolLit(bool),
    #[regex(r"[+-]?([0-9]*[.])?[0-9]+", |lex| lex.slice())]
    NumLit(&'src str),
    #[regex(r#""(\.|[^"\\])*""#, |lex| lex.slice())]
    StrLit(&'src str),
    #[regex(r"(\p{XID_Start}\p{XID_Continue}*)|(_\p{XID_Continue}+)", |lex| lex.slice())]
    Ident(&'src str),
    #[token("@")]
    At,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("=")]
    Eq,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(";")]
    Semi,
    #[token("->")]
    MinusGt,
    #[token("=>")]
    EqGt,
    #[token("||")]
    BarBar,
    #[token("&&")]
    AmpAmp,
    #[token("!")]
    Ex,
    #[token("==")]
    EqEq,
    #[token("!=")]
    ExEq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    LtEq,
    #[token(">=")]
    GtEq,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("^")]
    Caret,
    #[token("&")]
    Amp,
    #[token("|")]
    Bar,
    #[token("<<")]
    LtLt,
    #[token(">>")]
    GtGt,
    #[token("+=")]
    PlusEq,
    #[token("-=")]
    MinusEq,
    #[token("*=")]
    StarEq,
    #[token("/=")]
    SlashEq,
    #[token("%=")]
    PercentEq,
    #[token("^=")]
    CaretEq,
    #[token("&=")]
    AmpEq,
    #[token("|=")]
    BarEq,
    #[token("<<=")]
    LtLtEq,
    #[token(">>=")]
    GtGtEq,
    #[regex("[ \t\n\r]+")]
    Whitespace,
    #[regex("//[^\n]*")]
    Comment,
    #[token("mod")]
    Mod,
    #[token("import")]
    Import,
    #[token("component")]
    Component,
    #[token("system")]
    System,
    #[token("query")]
    Query,
    #[token("mut")]
    Mut,
    #[default]
    Error,
}
