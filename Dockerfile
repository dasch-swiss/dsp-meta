FROM rust:1-slim-bookworm as builder
WORKDIR /usr/src/dsp-meta
COPY . .
RUN cargo install --path ./dsp-meta-server

FROM debian:bookworm-slim AS runtime
# add data
COPY ./data ./data
# don't forget to set the env variable needed by the server
ENV DSP_META_DATA_DIR=./data
COPY --from=builder /usr/local/cargo/bin/dsp-meta-server /usr/local/bin/dsp-meta-server
CMD ["dsp-meta-server"]
