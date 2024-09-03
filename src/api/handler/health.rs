use tracing::trace;

pub(crate) async fn health() -> &'static str {
    trace!("entered health_handler()");
    "healthy"
}
