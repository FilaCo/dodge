use ariadne::{sources, Report, Source as ReportSource};
use dgc::prelude::*;
use std::ops::Range;
use std::{env, fs};

fn main() {
    let db = DodgeCompilerDatabase::default();

    let file_path = env::args().nth(1).expect("no input");
    let file_content = fs::read_to_string(&file_path).expect("unable to read file");
    let src = Source::new(&db, file_path.clone().into(), file_content.clone());

    compile(&db, src);
    compile::compile::accumulated::<Diagnostic>(&db, src)
        .into_iter()
        .for_each(|diag: &Diagnostic| {
            Report::build(
                diag.kind.into(),
                (file_path.as_str(), diag.span.start..diag.span.end),
            )
            .with_message(&diag.msg)
            // .with_labels(&diag.labels)
            .finish()
            .eprint((file_path.as_str(), ReportSource::from(src.content(&db))))
            .expect("report print failed");
        });
}
