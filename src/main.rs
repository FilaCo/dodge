use racketc::v1::{RacketCompiler, RacketResult};

fn main() -> RacketResult<()> {
    RacketCompiler::new().run()
}
