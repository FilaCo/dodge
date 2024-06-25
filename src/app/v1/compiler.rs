use crate::v1::RacketResult;

#[derive(Debug)]
pub struct RacketCompiler;

impl RacketCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> RacketResult<()> {
        Ok(())
    }
}

impl Default for RacketCompiler {
    fn default() -> Self {
        Self::new()
    }
}
