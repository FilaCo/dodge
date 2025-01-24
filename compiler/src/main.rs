use clap::Parser;
use dodgec::{CompileArgs, CompileError, DodgeCompiler};
use std::path::PathBuf;

fn main() -> Result<(), CompileError> {
    let cli = DodgeCompilerCli::parse();

    DodgeCompiler::new().compile(cli.into())
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct DodgeCompilerCli {
    package_root_filepath: PathBuf,
    #[arg(short, long)]
    threads_number: Option<u8>,
}

impl From<DodgeCompilerCli> for CompileArgs {
    fn from(value: DodgeCompilerCli) -> Self {
        let threads_number = value.threads_number.map(|nt| nt.into()).unwrap_or_default();

        Self {
            package_root_filepath: value.package_root_filepath,
            threads_number,
        }
    }
}
