# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

DSP-META is a service that provides metadata for the DaSCH Service Platform (DSP). It's a Rust-based web service with a Svelte frontend that manages project metadata in JSON format.

## Development Commands

The project uses `just` as the task runner. Key commands:

### Rust Backend
- `just build` - Build all Rust targets
- `just test` - Run all tests
- `just serve` - Build frontend and run the server (requires environment setup)
- `just serve-dev` - Run server with file watching for development
- `just watch` - Watch for changes and run tests
- `just check` - Run all format and clippy checks
- `just fmt` - Format all Rust code
- `just clean` - Clean all build artifacts

### Frontend (Svelte)
- `just build-frontend` - Build the web frontend
- `just serve-frontend` - Run the frontend development server
- `cd web-frontend && yarn fmt` - Format frontend code

### Documentation
- `just docs-serve` - Serve documentation locally with MkDocs
- `just docs-build` - Build documentation

## Architecture

### Backend Structure
- **main-server.rs**: HTTP server entry point using Axum framework
- **main-validator.rs**: CLI tool entry point for metadata validation
- **api/**: Web API layer with handlers and routing
  - `router.rs`: Main HTTP router configuration with CORS and tracing
  - `handler/`: HTTP request handlers organized by API version
- **domain/**: Core business logic
  - `metadata_service.rs`: Service layer for metadata operations
  - `metadata_repository.rs`: Data access layer for JSON metadata files
  - `model/`: Domain models and validation logic
- **app_state.rs**: Shared application state

### Frontend Structure
- Svelte-based single-page application in `web-frontend/`
- Main components: ProjectPage, ProjectsRepository with pagination
- Built with Rollup bundler

### Data Organization
- Project metadata stored as JSON files in `data/json/` directory
- Each project has a unique shortcode identifier
- JSON schema validation against predefined schemas in `resources/`

## Environment Configuration

The server uses environment variables with `DSP_META_` prefix:
- `DSP_META_DATA_DIR`: Path to metadata JSON files (default: `/data`)
- `DSP_META_PUBLIC_DIR`: Path to static frontend files (default: `/public`)
- `DSP_META_BASE_URL`: Base URL for the service (default: `https://meta.dasch.swiss`)
- `DSP_META_LOG_FILTER`: Log level configuration
- `DSP_META_LOG_FMT`: Log format (`json` for JSON output)

## Testing

- Use `cargo test --tests` for Rust unit/integration tests
- API tests are in `tests/api_tests.rs`
- Schema validation tests in `tests/validate_json_data_with_schema_test.rs`

## Commit Conventions

Uses Conventional Commits with specific prefixes mapped to changelog sections:
- `data:` - Project Data changes
- `feat:` - Enhancements  
- `fix:` - Bug Fixes
- `docs:` - Documentation
- `build:`, `chore:`, `refactor:` - Maintenances
