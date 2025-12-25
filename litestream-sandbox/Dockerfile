# syntax=docker/dockerfile:1
FROM rust:1.79.0-bookworm AS builder

WORKDIR /app

RUN curl -o /tmp/sccache.tgz -L https://github.com/mozilla/sccache/releases/download/0.2.13/sccache-0.2.13-x86_64-unknown-linux-musl.tar.gz && \
  tar xf /tmp/sccache.tgz -C /tmp && \
  mv /tmp/sccache*/sccache /usr/local/bin && \
  rm -rf /tmp/sccache*

RUN \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  apt-get update && apt-get install -y \
  libsqlite3-dev

ADD https://github.com/benbjohnson/litestream/releases/download/v0.3.13/litestream-v0.3.13-linux-amd64.tar.gz /tmp/litestream.tar.gz
RUN tar -C ./ -xzf /tmp/litestream.tar.gz

COPY . .

ENV CARGO_HOME=/var/cache/cargo
ENV RUSTC_WRAPPER=/usr/local/bin/sccache
ENV SCCACHE_DIR=/var/cache/sccache

#RUN \
#  --mount=type=cache,target=/var/cache/cargo \ 
#  cargo install -f sqlx-cli --no-default-features --features sqlite
#RUN cargo sqlx migrate run

RUN \
  --mount=type=cache,target=/var/cache/cargo \ 
  cargo fetch --locked

RUN \
  # --mount=type=cache,target=./target \
  --mount=type=cache,target=/var/cache/cargo \
  --mount=type=cache,target=/var/cache/sccache \
  cargo build --offline --release

FROM debian:bookworm-slim

WORKDIR /app

RUN \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  --mount=type=cache,target=/var/cache/apt,sharing=locked \
  apt-get update && apt-get install -y \
  ca-certificates openssl

COPY --from=builder /app/litestream /app/litestream
COPY --from=builder /app/litestream /app/litestream
COPY --from=builder /app/key.json /app/key.json
COPY --from=builder /app/litestream.yml /app/litestream.yml
COPY --from=builder /app/run.bash /app/run.bash
COPY --from=builder /app/.env /app/.env
COPY --from=builder /app/target/release/litestream-sandbox /app/target/release/litestream-sandbox
COPY --from=builder /app/web/dist /app/dist

EXPOSE 8080
CMD ["/app/run.bash"]
