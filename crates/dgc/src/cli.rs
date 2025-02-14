use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct DodgeCompilerCli {
    pub input: PathBuf,
}
