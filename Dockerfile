FROM rust:1-slim-bookworm as builder-rs
WORKDIR /dsp-meta
COPY . .
RUN cargo install --path ./dsp-meta-cmd

FROM node:21-bookworm-slim as builder-node
WORKDIR /dsp-meta
COPY . .
RUN cd web-frontend && yarn install && yarn run build

FROM debian:bookworm-slim AS runtime
# add data
COPY ./data /data

# don't forget to set the env variable needed by the server
ENV DSP_META_DATA_DIR=/data

COPY --from=builder-rs /usr/local/cargo/bin/dsp-meta-server /usr/local/bin/dsp-meta-server

COPY --from=builder-node /dsp-meta/web-frontend/public /public
ENV DSP_META_PUBLIC_DIR=/public

# set logging level
ENV DSP_META_LOG="info,hyper=info"

EXPOSE 3000
ENTRYPOINT ["dsp-meta-server"]
