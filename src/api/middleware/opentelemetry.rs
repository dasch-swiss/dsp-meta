use axum::extract::Request;
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use opentelemetry::propagation::Extractor;

/// Extractor adapter for Axum's HeaderMap to work with OpenTelemetry's propagation API
struct HeaderExtractor<'a>(&'a HeaderMap);

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

/// Middleware that extracts OpenTelemetry trace context from incoming HTTP headers.
///
/// This middleware:
/// - Extracts W3C TraceContext (traceparent/tracestate headers) from the request
/// - Establishes the extracted context as the parent for all spans created during request processing
/// - Falls back gracefully when no trace context is present (creates new root spans)
///
/// In production with a reverse proxy, the proxy injects traceparent headers and this middleware
/// ensures your application continues the distributed trace.
///
/// In local development without a proxy, new root traces are created automatically.
pub async fn extract_trace_context(request: Request, next: Next) -> Response {
    // Extract the trace context from HTTP headers using the global propagator
    let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(request.headers()))
    });

    // Attach the context for the duration of the request
    // This ensures any spans created during request processing use this context
    let _guard = parent_context.attach();
    next.run(request).await
}
