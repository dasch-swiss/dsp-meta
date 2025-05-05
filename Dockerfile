FROM rust:1-slim-bookworm AS builder-rs
WORKDIR /dsp-meta
COPY . .
RUN cargo install --path ./

FROM node:21-bookworm-slim AS builder-node
WORKDIR /dsp-meta
COPY . .
RUN cd web-frontend && yarn install && yarn run build --bundleConfigAsCjs

FROM debian:bookworm-slim AS runtime
# add data
COPY ./data /data

# don't forget to set the env variable needed by the server
ENV DSP_META_DATA_DIR=/data

COPY --from=builder-rs /usr/local/cargo/bin/dsp-meta /usr/local/bin/dsp-meta

COPY --from=builder-node /dsp-meta/web-frontend/public /public
ENV DSP_META_PUBLIC_DIR=/public

# set logging level
ENV DSP_META_LOG_FILTER="info,hyper=info"

# set log output type
ENV DSP_META_LOG_FMT="json"

# enable periodic container health check
HEALTHCHECK --start-period=60s --interval=60s --timeout=10s --retries=3 \
  CMD curl -sS --fail "http://127.0.0.1:3000/health" | grep '^healthy$' || exit 1

EXPOSE 3000
ENTRYPOINT ["dsp-meta"]
