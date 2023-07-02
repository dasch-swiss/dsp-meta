mod cli;

use env_logger::{Env, Target};
use log::trace;

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
        .target(Target::Stdout)
        .init();

    trace!("Hello, world!");

    cli::parse()?;
    anyhow::Ok(())
}
