# List all recipies
default:
	just --list --unsorted

check:
    cargo +nightly fmt --check
    cargo clippy -- -D warnings

build:
    cargo build

test:
    cargo test

run:
    export DSP_META_DATA_DIR=${PWD}/data && cargo run --bin dsp_meta_server
