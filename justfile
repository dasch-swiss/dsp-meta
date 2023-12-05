DOCKER_REPO := "daschswiss/dsp-meta-server"
BUILD_TAG := `git describe --tag --dirty=-changed --abbrev=7 --always`
DOCKER_IMAGE := DOCKER_REPO + ":" + BUILD_TAG

# List all recipies
default:
    just --list --unsorted

# Run all fmt and clippy checks
check:
    just --check --fmt --unstable
    cargo +nightly fmt --check
    cargo clippy -- -D warnings

# Fix justfile formating. Warning: will change existing file. Please first use check.
fix:
    just --fmt --unstable

# Run all builds
build:
    cargo build

# Run all tests
test:
    cargo test

# Run dsp-meta-server
run:
    export DSP_META_DATA_DIR=${PWD}/data && cargo run --bin dsp-meta-server

# Run dsp-meta-validator validating all hcl documents under ./data
validate:
    export DSP_META_DATA_DIR=${PWD}/data && cargo run --bin dsp-meta-validator

# build linux/amd64 Docker image locally
docker-build-amd64:
    docker buildx build --platform linux/amd64 -t {{ DOCKER_IMAGE }}-amd64 -t {{ DOCKER_REPO }}:latest --load .

# push previously build linux/amd64 image to Docker hub
docker-push-amd64:
    docker push {{ DOCKER_IMAGE }}-amd64

# build linux/arm64 Docker image locally
docker-build-arm64:
    docker buildx build --platform linux/arm64 -t {{ DOCKER_IMAGE }}-arm64 -t {{ DOCKER_REPO }}:latest --load .

# push previously build linux/arm64 image to Docker hub
docker-push-arm64:
    docker push {{ DOCKER_IMAGE }}-arm64

# publish Docker manifest combining aarch64 and x86 published images
docker-publish-manifest:
    docker manifest create {{ DOCKER_IMAGE }} --amend {{ DOCKER_IMAGE }}-amd64 --amend {{ DOCKER_IMAGE }}-arm64
    docker manifest annotate --arch amd64 --os linux {{ DOCKER_IMAGE }} {{ DOCKER_IMAGE }}-amd64
    docker manifest annotate --arch arm64 --os linux {{ DOCKER_IMAGE }} {{ DOCKER_IMAGE }}-arm64
    docker manifest inspect {{ DOCKER_IMAGE }}
    docker manifest push {{ DOCKER_IMAGE }}
