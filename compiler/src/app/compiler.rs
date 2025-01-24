use crate::prelude::CompileArgs;
use thiserror::Error;

#[derive(Debug)]
pub struct DodgeCompiler {}

impl DodgeCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, args: CompileArgs) -> Result<(), CompileError> {
        let CompileArgs {
            package_root_filepath: _package_root_filepath,
            threads_number: _threads_number,
        } = args;

        Ok(())
    }
}

impl Default for DodgeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error)]
pub enum CompileError {}
