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
