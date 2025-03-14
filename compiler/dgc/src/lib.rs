#![feature(allocator_api)]
#![no_std]
extern crate alloc as core_alloc;

mod alloc;
mod core;

// #[derive(Debug)]
// pub struct Node {
//     pub kind: NodeKind,
// }
//
// #[derive(Debug)]
// pub enum NodeKind {
//     /// A module declaration (`mod`).
//     ///
//     /// E.g. `mod foo;` or `mod foo { ... }`.
//     Mod,
//     /// A component declaration (`component`).
//     ///
//     /// E.g. `component Foo;` or `component Foo { ... }`.
//     Component,
//     /// A system declaration (`system`).
//     ///
//     /// E.g. `system HandleFoo { ... }`.
//     System,
//     /// A import declaration (`import`)
//     ///
//     /// E.g. `import foo.bar` or `import foo.baz.*`
//     Import,
//     /// A event declaration (`event`)
//     ///
//     /// E.g. `event Foo;` or `event Bar { ... }`
//     Event,
//     /// A const declaration (`const`)
//     ///
//     /// E.g. `const G: f64 = 9.8;`
//     Const,
// }
//
// /// Parsed token.
// /// It doesn't contain information about data that has been parsed,
// /// only the type of the token and its size.
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub struct Token {
//     pub kind: TokenKind,
//     pub len: usize,
// }
//
// impl Token {
//     pub fn new(kind: TokenKind, len: usize) -> Self {
//         Self { kind, len }
//     }
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub enum TokenKind {
//     /// A block comment, e.g. `/* block comment */`.
//     ///
//     /// Block comments can be recursive, so a sequence like `/* /* */`
//     /// will not be considered terminated and will result in a parsing error.
//     BlockComment { is_terminated: bool },
//
//     /// End of input.
//     Eof,
//
//     /// An identifier or keyword, e.g. `ident` or `component`.
//     Ident,
//
//     /// A line comment, e.g. `// comment`.
//     LineComment,
//
//     /// Literals, e.g. `12u8`, `1.0e-40`, `"123"`.
//     ///
//     /// See [LitKind] for more details.
//     Lit { kind: LitKind, suffix_start: usize },
//
//     /// A raw identifier, e.g. `` `ident` ``.
//     RawIdent,
//
//     /// `;`
//     Semi,
//     /// `,`
//     Comma,
//     /// `.`
//     Dot,
//     /// `(`
//     OpenParen,
//     /// `)`
//     CloseParen,
//     /// `{`
//     OpenBrace,
//     /// `}`
//     CloseBrace,
//     /// `[`
//     OpenBracket,
//     /// `]`
//     CloseBracket,
//     /// `@`
//     At,
//     /// `#`
//     Hash,
//     /// `~`
//     Tilde,
//     /// `?`
//     Question,
//     /// `:`
//     Colon,
//     /// `$`
//     Dollar,
//     /// `=`
//     Eq,
//     /// `!`
//     Exclamation,
//     /// `<`
//     Lt,
//     /// `>`
//     Gt,
//     /// `-`
//     Minus,
//     /// `&`
//     Amp,
//     /// `|`
//     Pipe,
//     /// `+`
//     Plus,
//     /// `*`
//     Star,
//     /// `/`
//     Slash,
//     /// `^`
//     Caret,
//     /// `%`
//     Percent,
// }
//
// /// Enum representing the literal types supported by the lexer.
// ///
// /// Note that the suffix is *not* considered when deciding the `LitKind` in
// /// this type. This means that float literals like `1f32` are classified by this
// /// type as `Int`.
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub enum LitKind {
//     /// `12_u8`, `0o100`, `0b120i99`, `1f32`.
//     Int { base: Base },
//     /// `12.34f32`, `1e3`, but not `1f32`.
//     Float { base: Base },
//     /// `'a'`, `'\\'`, `'''`, `';`
//     Rune { is_terminated: bool },
//     /// `"abc"`, `"abc`
//     Str { is_terminated: bool },
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub enum Base {
//     /// Literal starts with `0b` or `0B`.
//     Bin = 2,
//     /// Literal starts with `0o` or `0O`.
//     Oct = 8,
//     /// Literal doesn't contain a prefix.
//     Dec = 10,
//     /// Literal starts with `0x` or `0X`.
//     Hex = 16,
// }
