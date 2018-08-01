FROM ubuntu:18.04

RUN apt-get update -y
RUN apt-get install --force-yes -y --no-install-recommends \
    binutils curl wget zip git vim \
    build-essential \
    ca-certificates \
    file \
    pkg-config \
    musl-tools
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH $PATH:/root/.cargo/bin
RUN rustup target add x86_64-unknown-linux-musl
# cache
RUN cargo search cargo


WORKDIR /source

ADD . /source
RUN cargo +nightly build --target=x86_64-unknown-linux-musl --release

FROM alpine:edge

COPY --from=builder /source/target/x86_64-unknown-linux-musl/release/musl-test /opt/server
CMD ["/opt/server"]