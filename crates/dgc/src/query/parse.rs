use crate::ir::{Ast, Source};
use chumsky::prelude::{Simple, filter, just, one_of, take_until};
use chumsky::recovery::skip_then_retry_until;
use chumsky::text::TextParser;
use chumsky::{Parser, text};
use salsa::Database;

#[salsa::tracked]
pub fn parse(db: &dyn Database, src: Source) -> Ast<'_> {
    todo!()
}

const MOD_KW: &str = "mod";
const COMPONENT_KW: &str = "component";
const SYSTEM_KW: &str = "system";
const IMPORT_KW: &str = "import";
const QUERY_KW: &str = "query";
const WHERE_KW: &str = "where";
const WITH_KW: &str = "with";
const WITHOUT_KW: &str = "without";
const IF_KW: &str = "if";
const ELSE_KW: &str = "else";
const MATCH_KW: &str = "match";

const TRUE_BOOL_LIT: &str = "true";
const FALSE_BOOL_LIT: &str = "false";

const OPS: &str = "+-*/%&|^=<>?!";
const PLUS: char = '+';
const MINUS: char = '-';
const STAR: char = '*';
const SLASH: char = '/';
const PERCENT: char = '%';
const AND: char = '&';
const OR: char = '|';
const CARET: char = '^';
const EQ: char = '=';
const LT: char = '<';
const GT: char = '>';
const QUESTION: char = '?';
const EX: char = '!';

const PLUS_PLUS: &str = "++";
const MINUS_MINUS: &str = "--";
const AND_AND: &str = "&&";
const OR_OR: &str = "||";
const EQ_EQ: &str = "==";
const EQ_GT: &str = "=>";
const EX_EQ: &str = "!=";
const LT_LT: &str = "<<";
const LT_EQ: &str = "<=";
const GT_GT: &str = ">>";
const GT_EQ: &str = ">=";
const COLON_COLON: &str = "::";

const PUNC_SYMBOLS: &str = ".,:;{}[]()";
const DOT: char = '.';
const COMMA: char = ',';
const COLON: char = ':';
const SEMI: char = ';';
const QUOTE: char = '"';
const OPEN_BRACE: char = '{';
const CLOSE_BRACE: char = '}';
const OPEN_BRACKET: char = '[';
const CLOSE_BRACKET: char = ']';
const OPEN_PAREN: char = '(';
const CLOSE_PAREN: char = ')';

#[derive(Debug, Clone, PartialEq)]
enum Token {
    /// An identifier e.g. `ident`.
    Id(String),

    /// [MOD_KW]
    Mod,
    /// [COMPONENT_KW]
    Component,
    /// [SYSTEM_KW]
    System,
    /// [IMPORT_KW]
    Import,
    /// [QUERY_KW]
    Query,
    /// [WHERE_KW]
    Where,
    /// [WITH_KW]
    With,
    /// [WITHOUT_KW]
    Without,
    /// [IF_KW]
    If,
    /// [ELSE_KW]
    Else,
    /// [MATCH_KW]
    Match,

    /// A line comment, e.g. `// comment`.
    LineComment,
    /// A block comment, e.g. `/* block comment */`.
    ///
    /// Block comments can be recursive, so a sequence like `/* /* */`
    /// will not be considered terminated and will result in a parsing error.
    BlockComment,

    /// A number literal, e.g. `42`
    NumLit(String),
    /// A string literal, e.g. `"some string"`
    StrLit(String),
    /// A bool literal, e.g. [TRUE_BOOL_LIT] or [FALSE_BOOL_LIT]
    BoolLit(bool),

    /// [PLUS]
    Plus,
    /// [MINUS]
    Minus,
    /// [STAR]
    Star,
    /// [SLASH]
    Slash,
    /// [PERCENT]
    Percent,
    /// [AND]
    And,
    /// [OR]
    Or,
    /// [CARET]
    Caret,
    /// [EQ]
    Eq,
    /// [LT]
    Lt,
    /// [GT]
    Gt,
    /// [QUESTION]
    Question,
    /// [EX]
    Ex,
    /// [PLUS_PLUS]
    PlusPlus,
    /// [MINUS_MINUS]
    MinusMinus,
    /// [AND_AND]
    AndAnd,
    /// [OR_OR]
    OrOr,
    /// [EQ_EQ]
    EqEq,
    /// [EQ_GT]
    EqGt,
    /// [EX_EQ]
    ExEq,
    /// [LT_LT]
    LtLt,
    /// [LT_EQ]
    LtEq,
    /// [GT_GT]
    GtGt,
    /// [GT_EQ]
    GtEq,
    /// [COLON_COLON]
    ColonColon,

    /// [DOT]
    Dot,
    /// [COMMA]
    Comma,
    /// [COLON]
    Colon,
    /// [SEMI]
    Semi,
    /// [QUOTE]
    Quote,
    /// [OPEN_BRACE]
    OpenBrace,
    /// [CLOSE_BRACE]
    CloseBrace,
    /// [OPEN_BRACKET]
    OpenBracket,
    /// [CLOSE_BRACKET]
    CloseBracket,
    /// [OPEN_PAREN]
    OpenParen,
    /// [CLOSE_PAREN]
    CloseParen,

    Invalid(String),
}

type Span = std::ops::Range<usize>;

fn chumsky_lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let ident = text::ident().map(|lexeme: String| match lexeme.as_str() {
        MOD_KW => Token::Mod,
        COMPONENT_KW => Token::Component,
        SYSTEM_KW => Token::System,
        IMPORT_KW => Token::Import,
        QUERY_KW => Token::Query,
        WHERE_KW => Token::Where,
        WITH_KW => Token::With,
        WITHOUT_KW => Token::Without,
        IF_KW => Token::If,
        ELSE_KW => Token::Else,
        MATCH_KW => Token::Match,
        _ => Token::Id(lexeme),
    });

    let bool_lit = text::keyword()
    let num_lit = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(Token::NumLit);

    let str_lit = just(QUOTE)
        .ignore_then(filter(|c| *c != QUOTE).repeated())
        .then_ignore(just(QUOTE))
        .collect::<String>()
        .map(Token::StrLit);

    let op = one_of(OPS)
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|lexeme: String| match lexeme.as_str() {
            PLUS_PLUS => Token::PlusPlus,
            MINUS_MINUS => Token::MinusMinus,
            AND_AND => Token::AndAnd,
            OR_OR => Token::OrOr,
            EQ_EQ => Token::EqEq,
            EQ_GT => Token::EqGt,
            EX_EQ => Token::EqEq,
            LT_LT => Token::LtLt,
            LT_EQ => Token::LtEq,
            GT_GT => Token::GtGt,
            GT_EQ => Token::GtEq,
            COLON_COLON => Token::ColonColon,
            _ => Token::Invalid(lexeme),
        });

    let token = num_lit
        .or(str_lit)
        .or(op)
        .or(ident_or_bool_lit)
        .recover_with(skip_then_retry_until([]));

    let line_comment = just("//").then(take_until(just('\n'))).padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(line_comment.repeated())
        .padded()
        .repeated()
}
