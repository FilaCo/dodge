use dgc::prelude::*;
use std::{env, fs};

fn main() {
    let db = DodgeCompilerDatabase::default();

    let file_path = env::args().nth(1).expect("no input");
    let src = Source::new(
        &db,
        file_path.clone().into(),
        fs::read_to_string(&file_path)
            .expect("unable to read file")
            .into(),
    );

    query::lex(&db, src);
}
