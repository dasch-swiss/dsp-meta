use tracing::trace;

pub(crate) async fn health_handler() -> &'static str {
    trace!("entered health_handler()");
    "healthy"
}
