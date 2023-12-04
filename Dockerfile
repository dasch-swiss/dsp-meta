FROM rust:1-slim-bookworm as chef
RUN cargo install cargo-chef
WORKDIR /usr/src/dsp-meta

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /usr/src/dsp-meta/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo install --path ./dsp-meta-server

FROM debian:bookworm-slim AS runtime
# add data
COPY ./data ./data
# don't forget to set the env variable needed by the server
ENV DSP_META_DATA_DIR=./data
COPY --from=builder /usr/local/cargo/bin/dsp-meta-server /usr/local/bin/dsp-meta-server
CMD ["dsp-meta-server"]
