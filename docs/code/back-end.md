# Back-End

The back-end of the DSP Metadata Browser is built with Rust using the [Axum](https://github.com/tokio-rs/axum) web framework.

## Architecture

### Core Components

- **HTTP Server** ([src/main-server.rs](../../src/main-server.rs)): Entry point using Axum framework with async Tokio runtime
- **API Router** ([src/api/router.rs](../../src/api/router.rs)): HTTP routing, CORS, and tracing middleware
- **Domain Layer** ([src/domain/](../../src/domain/)): Business logic and data access
    - `metadata_service.rs`: Service layer for metadata operations
    - `metadata_repository.rs`: Data access for JSON metadata files
    - `model/`: Domain models and validation
- **Handlers** ([src/api/handler/](../../src/api/handler/)): HTTP request handlers organized by API version

### Technology Stack

- **Framework**: [Axum](https://docs.rs/axum/) - Ergonomic web framework built on Tokio
- **Runtime**: [Tokio](https://tokio.rs/) - Asynchronous runtime
- **Serialization**: [Serde](https://serde.rs/) - JSON serialization/deserialization
- **Validation**: [Valico](https://docs.rs/valico/) - JSON schema validation
- **Tracing**: [tracing](https://docs.rs/tracing/) + [OpenTelemetry](https://opentelemetry.io/) - Observability

## Key Features

### Metadata Management

The backend serves project metadata stored as JSON files. Each project is identified by a unique shortcode and
validated against JSON schemas defined in `resources/`.

### API Endpoints

- `GET /api/v1/projects` - List projects with pagination and filtering
- `GET /api/v1/projects/{shortcode}` - Get specific project by shortcode
- `GET /health` - Health check endpoint
- `GET /version.txt` - Current application version

### Configuration

The application is configured via environment variables with the `DSP_META_` prefix:

| Variable | Description | Default |
|----------|-------------|---------|
| `DSP_META_DATA_DIR` | Path to metadata JSON files | `/data` |
| `DSP_META_PUBLIC_DIR` | Path to static frontend files | `/public` |
| `DSP_META_BASE_URL` | Base URL for the service | `https://meta.dasch.swiss` |
| `DSP_META_LOG_FILTER` | Log level filter | `info` |
| `DSP_META_LOG_FMT` | Log format (json/compact) | `compact` |
| `DSP_META_OTLP_ENDPOINT` | OpenTelemetry OTLP endpoint | Not set |

### Observability

The backend implements comprehensive distributed tracing using OpenTelemetry:

- **Automatic span creation** for HTTP requests and instrumented functions
- **W3C TraceContext propagation** from reverse proxies and upstream services
- **Optional OTLP export** to observability backends like Grafana Tempo

See [Observability & Tracing](observability.md) for detailed documentation on testing and configuring distributed tracing.

## Development

### Building

```bash
just build
```

### Running Locally

```bash
just serve-dev
```

### Testing

```bash
just test
```

### Code Quality

```bash
# Format code
just fmt

# Run linters
just check
```

!!! info

    More documentation will be added progressively.
