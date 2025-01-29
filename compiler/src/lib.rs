pub fn compile(src: Source) -> Result<(), CompileError> {
    let ast = parse(src)?;
    let ir = generate_ir(ast)?;
    compile_ir(ir)?;

    Ok(())
}

fn parse(src: Source) -> Result<AST, CompileError> {
    todo!()
}

fn generate_ir(ast: AST) -> Result<IR, CompileError> {
    todo!()
}

fn compile_ir(ir: IR) -> Result<(), CompileError> {
    todo!()
}

#[derive(Debug)]
pub struct Source;

#[derive(Debug)]
pub struct IR;

#[derive(Debug)]
pub struct CompileError;

#[derive(Debug)]
struct AST;

#[derive(Debug)]
struct ASTNode {}

#[derive(Debug)]
struct Token {
    pub kind: TokenKind,
}

#[derive(Debug)]
enum TokenKind {
    Eof,
    Lit,
    Ident,
    Whitespace,

    LineComment,
    BlockComment,

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

    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `=`
    Eq,
}

#[derive(Debug)]
struct Lexer {}

impl Lexer {}
