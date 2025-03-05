// use crate::ir::{Ast, ExprKind, Source};
// use crate::prelude::{AstNode, TokenKind};
// use crate::query;
// use chumsky::prelude::{just, recursive};
// use chumsky::{Parser, select};
// use clap::builder::TypedValueParser;
// use salsa::Database;
// use std::io::Read;
//
// #[salsa::tracked]
// pub fn parse(db: &dyn Database, src: Source) -> Ast<'_> {
//     let lexemes = query::lex(db, src);
//
//     todo!()
// }
//
// fn chumsky_parser<'src>() {
//     mod_decl.or()
//     let lit_expr = select! {
//         TokenKind::BoolLit(b) => AstNode::Expr(ExprKind::BoolLit(b)),
//         TokenKind::StrLit(s) => AstNode::Expr(ExprKind::StrLit(s)),
//         TokenKind::NumLit(s) => AstNode::Expr(ExprKind::NumLit(s)),
//     };
// }
