mod cli;

use dsp_meta::error::DspMetaError;
use tracing::{trace, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), DspMetaError> {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    trace!("Ivan was here!");

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    trace!("Version: {}", VERSION);

    cli::parse()?;
    Ok(())
}
