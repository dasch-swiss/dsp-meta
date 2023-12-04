FROM rust:1-slim-bookworm as builder
WORKDIR /usr/src/dsp-meta
COPY . .
RUN cargo install --path ./dsp-meta-server

FROM rust:1-slim-bookworm
COPY ./data ./data
COPY --from=builder /usr/local/cargo/bin/dsp-meta-server /usr/local/bin/dsp-meta-server
CMD ["dsp-meta-server"]
