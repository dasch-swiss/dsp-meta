DOCKER_REPO := "daschswiss/dsp-meta-server"
CARGO_VERSION := `cargo metadata --format-version=1 --no-deps | jq --raw-output '.packages[].version'`
COMMIT_HASH := `git log --pretty=format:'%h' -n 1`
GIT_TAG := `git describe --tags --exact-match 2>/dev/null || true`
IMAGE_TAG := if GIT_TAG == "" { CARGO_VERSION + "-" + COMMIT_HASH } else { GIT_TAG }
DOCKER_IMAGE := DOCKER_REPO + ":" + IMAGE_TAG

# List all recipies
default:
    just --list --unsorted

# Run all fmt and clippy checks
check:
    just --check --fmt --unstable
    cargo +nightly fmt --check
    cargo clippy -- -D warnings

# Format all rust code
fmt:
    cargo +nightly fmt
    cd web-frontend && yarn fmt

# Fix justfile formating. Warning: will change existing file. Please first use check.
fix:
    just --fmt --unstable

# Run all rust builds
build:
    cargo build --all-targets

# Build web-frontend
build-frontend:
    cd web-frontend && yarn install && yarn run build

# Run all tests
test:
    cargo test --tests

# Clean all build artifacts
clean:
    cargo clean

# Run dsp-meta-server
serve: build-frontend clean
    export DSP_META_DATA_DIR=${PWD}/data && export DSP_META_PUBLIC_DIR=${PWD}/web-frontend/public && export DSP_META_LOG_FILTER=trace,hyper=info && cargo run --bin dsp-meta-server

# Run dsp-meta-server and watch for changes in the Rust code
serve-dev:
    export DSP_META_DATA_DIR=${PWD}/data && export DSP_META_PUBLIC_DIR=${PWD}/web-frontend/public && export DSP_META_LOG_FILTER=trace,hyper=info && cargo watch -x 'run --bin dsp-meta-server'

# Run the frontend dev server
serve-frontend:
    cd web-frontend && yarn run dev

# Build linux/amd64 Docker image locally
docker-build-amd64:
    docker buildx build --platform linux/amd64 -t {{ DOCKER_IMAGE }}-amd64 --load .

# Push previously build linux/amd64 image to Docker hub
docker-push-amd64:
    docker push {{ DOCKER_IMAGE }}-amd64

# Build linux/arm64 Docker image locally
docker-build-arm64:
    docker buildx build --platform linux/arm64 -t {{ DOCKER_IMAGE }}-arm64 --load .

# Push previously build linux/arm64 image to Docker hub
docker-push-arm64:
    docker push {{ DOCKER_IMAGE }}-arm64

# Publish Docker manifest combining aarch64 and x86 published images
docker-publish-manifest:
    docker manifest create {{ DOCKER_IMAGE }} --amend {{ DOCKER_IMAGE }}-amd64 --amend {{ DOCKER_IMAGE }}-arm64
    docker manifest annotate --arch amd64 --os linux {{ DOCKER_IMAGE }} {{ DOCKER_IMAGE }}-amd64
    docker manifest annotate --arch arm64 --os linux {{ DOCKER_IMAGE }} {{ DOCKER_IMAGE }}-arm64
    docker manifest inspect {{ DOCKER_IMAGE }}
    docker manifest push {{ DOCKER_IMAGE }}

# Output the BUILD_TAG
docker-image-tag:
    @echo {{ IMAGE_TAG }}

# Watch for changes and run tests
watch:
    cargo watch -x test
