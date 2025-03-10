#[salsa::tracked]
pub struct Ast<'db> {
    #[tracked]
    #[return_ref]
    pub root: Option<AstNode<'db>>,
}

#[derive(salsa::Update, Debug)]
pub enum AstNode<'db> {
    // Decl(DeclKind<'db>),
    // Def(DefKind<'db>),
    Expr(ExprKind<'db>),
    // Stmt(StmtKind<'db>),
}

// #[derive(salsa::Update, Debug)]
// pub enum DeclKind<'db> {
//     Mod,
//     Component,
//     System,
//     Query,
// }

#[derive(salsa::Update, Debug)]
pub enum ExprKind<'db> {
    Val(Val<'db>),
}

// #[derive(Debug)]
// pub enum StmtKind<'db> {
//     Import,
// }
//
// #[derive(Debug)]
// pub enum DefKind<'db> {}

#[derive(salsa::Update, Debug)]
pub enum Val<'db> {
    Bool(bool),
    Num(NumVal<'db>),
    Str(StrVal<'db>),
}

#[salsa::interned]
pub struct Symbol<'db> {
    #[return_ref]
    pub value: String,
}

#[salsa::interned]
pub struct StrVal<'db> {
    #[return_ref]
    pub value: String,
}

#[salsa::interned]
pub struct NumVal<'db> {
    #[return_ref]
    pub value: String,
}
