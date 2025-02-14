use clap::Parser;
use dgc::prelude::*;
use std::fs;

fn main() {
    let args = DodgeCompilerCli::parse();
    let src_path = args.input.into();
    let src_contents = fs::read_to_string(&src_path).expect("unable to read source file");

    let db = DodgeCompilerDatabase::default();
    compile(&db, Source::new(&db, src_path, src_contents));
}
