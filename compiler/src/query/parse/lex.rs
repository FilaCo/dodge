use chumsky::prelude::{just, recursive};
use chumsky::{extra, Parser};
use salsa::Database;
use thiserror::Error;

#[derive(Error, Debug, Default, Copy, Clone, PartialEq)]
pub enum LexError {
    #[error("unterminated block comment")]
    UnTerminatedBlockComment,
    #[default]
    #[error("internal lexer error")]
    Internal,
}

pub fn lexer<'db>(db: &'db dyn Database) {
    // let block_comment = recursive::<_, _, extra::Err>(|bc| {
    //     bc.delimited_by(OPEN_BLOCK_COMMENT, CLOSE_BLOCK_COMMENT)
    // });

    // let comment = just(LINE_COMMENT).ignore_then()
}

const OPEN_BLOCK_COMMENT: &str = "/*";
const CLOSE_BLOCK_COMMENT: &str = "*/";
const LINE_COMMENT: &str = "//";
