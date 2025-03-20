use dgc::prelude::*;

fn main() -> Result<(), DgcError> {
    Dgc::builder()
        .with_envs()
        .with_cli()
        .with_defaults()
        .build()?
        .run()
}
