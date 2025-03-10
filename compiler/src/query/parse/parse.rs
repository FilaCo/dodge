use crate::prelude::{Ast, Source};
use salsa::Database;

#[salsa::tracked]
pub fn parse(db: &dyn Database, src: Source) -> Ast<'_> {
    // let tokens = lex(db, src);
    // for token in tokens.value(db).iter() {
    //     println!("{:?}", token);
    // }
    // let tok_stream = Stream::from_iter(tok_iter.into_iter().map(|(t, s)| {
    //     (
    //         t.expect("only valid tokens are expected after partition."),
    //         s,
    //     )
    // }))
    // .map(0..content.len().into(), |(t, s): (_, _)| (t, s.into()));
    Ast::new(db, None)
}
