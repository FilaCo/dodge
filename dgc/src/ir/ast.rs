// use crate::ir::Spanned;
// use std::sync::Arc;
//
// pub struct Ast {
//     pub nodes: Vec<Spanned<AstNode>>,
// }
//
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum AstNode {
//     Decl(DeclKind),
//     Def(DefKind),
//     Expr(ExprKind),
//     Stmt(StmtKind),
// }
//
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum DeclKind {
//     Mod,
//     Component,
//     System,
//     Query,
// }
//
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum ExprKind<'src> {
//     BoolLit(bool),
//     NumLit(&'src str),
//     StrLit(&'src str),
//     Ident,
//     Unary,
//     Binary,
// }
//
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum StmtKind {
//     Import,
// }
//
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum DefKind {}
