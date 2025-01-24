use clap::Parser;
use dodgec::prelude::*;
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
    threads_number: Option<usize>,
}

impl From<DodgeCompilerCli> for CompileArgs {
    fn from(value: DodgeCompilerCli) -> Self {
        let DodgeCompilerCli {
            package_root_filepath,
            threads_number,
        } = value;

        Self {
            package_root_filepath,
            threads_number,
        }
    }
}
