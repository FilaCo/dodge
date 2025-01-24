use crate::util::ThreadsNumber;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug)]
pub struct DodgeCompiler {}

impl DodgeCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, args: CompileArgs) -> Result<(), CompileError> {
        println!("{:?}", args);

        Ok(())
    }
}

impl Default for DodgeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct CompileArgs {
    pub package_root_filepath: PathBuf,
    pub threads_number: ThreadsNumber,
}

#[derive(Debug, Error)]
pub enum CompileError {}
