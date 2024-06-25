use racket_rs::v1::{RacketCompiler, RacketResult};

fn main() -> RacketResult<()> {
    RacketCompiler::new().run()
}
