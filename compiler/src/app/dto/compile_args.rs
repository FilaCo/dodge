use std::path::PathBuf;

#[derive(Debug)]
pub struct CompileArgs {
    pub package_root_filepath: PathBuf,
    pub threads_number: Option<usize>,
}
