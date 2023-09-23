use dsp_meta::errors::DspMetaError;
use tracing::{trace, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), DspMetaError> {
    // configure tracing library
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    trace!("Hello, world!");

    dsp_meta::operation::server::run();
    Ok(())
}
