use crate::prelude::DgcEvent;
use clap::Parser;
use dgc_ecs::prelude::WorldManager;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, LazyLock};
use thiserror::Error;

#[derive(Debug)]
pub struct Dgc {
    wm: Arc<WorldManager>,
}

impl Dgc {
    fn new() -> Self {
        todo!()
    }

    pub fn builder() -> DgcBuilder {
        DgcBuilder::default()
    }

    pub fn run(&self) -> Result<(), DgcError> {
        // loop {
        //     self.wm.run()
        // }
        todo!()
    }

    pub fn make_sender(&self) -> Result<Sender<DgcEvent>, DgcError> {
        todo!()
    }

    pub fn make_receiver(&self) -> Result<Receiver<DgcEvent>, DgcError> {
        todo!()
    }
}

#[derive(Error, Debug)]
#[error("oops, something went wrong")]
#[non_exhaustive]
pub struct DgcError(#[from] pub DgcErrorSource);

#[derive(Error, Debug)]
#[error(transparent)]
pub enum DgcErrorSource {
    Build(#[from] DgcBuilderError),
}

impl From<DgcBuilderError> for DgcError {
    fn from(value: DgcBuilderError) -> Self {
        DgcErrorSource::Build(value).into()
    }
}

#[derive(Debug)]
pub struct DgcBuilder {
    cli: LazyLock<Cli>,

    input: ReqArg<PathBuf>,
    output: OptArg<PathBuf>,
}

impl DgcBuilder {
    pub fn new() -> Self {
        Self {
            cli: LazyLock::new(Cli::parse),

            input: None,
            output: OptArg::Undefined,
        }
    }

    pub fn with_envs(mut self) -> Self {
        self
    }

    pub fn with_cli(mut self) -> Self {
        if self.input.is_none() {
            self.input = Some(self.cli.input.clone());
        }

        if self.output.is_undefined() {
            self.output = self
                .cli
                .output
                .clone()
                .map_or(OptArg::Undefined, |o| OptArg::Defined(Some(o)))
        }

        self
    }

    pub fn with_defaults(mut self) -> Self {
        self
    }

    pub fn with_input(mut self, input: PathBuf) -> Self {
        self.input = Some(input);

        self
    }

    pub fn with_output(mut self, output: Option<PathBuf>) -> Self {
        self.output = OptArg::Defined(output);

        self
    }

    pub fn build(self) -> Result<Dgc, DgcBuilderError> {
        Ok(Dgc::new())
    }
}

impl Default for DgcBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
#[error("unable to build the compiler")]
#[non_exhaustive]
pub struct DgcBuilderError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    pub input: PathBuf,
    /// Write output to <filename>
    #[clap(short, value_name = "FILENAME")]
    pub output: Option<PathBuf>,
}

#[derive(Debug)]
enum OptArg<T> {
    Defined(Option<T>),
    Undefined,
}

impl<T> OptArg<T> {
    fn is_undefined(&self) -> bool {
        matches!(self, OptArg::Undefined)
    }
}

type ReqArg<T> = Option<T>;
