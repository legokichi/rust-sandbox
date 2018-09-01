FROM messense/rust-musl-cross:x86_64-musl as builder

ADD . /home/rust/src
RUN cargo build --release
RUN readelf -d /home/rust/src/target/x86_64-unknown-linux-musl/release/alpine-sandbox

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/alpine-sandbox \
    /usr/local/bin/
CMD /usr/local/bin/alpine-sandbox