FROM debian:jessie as builder
ENV DEBIAN_FRONTEND noninteractive
ENV PKG_CONFIG_ALLOW_CROSS 1

RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install -y --no-install-recommends \
    binutils curl wget zip git vim \
    build-essential \
    ca-certificates \
    file \
    pkg-config \
    libssl-dev

RUN echo "deb [arch=armhf] http://archive.raspbian.org/raspbian jessie main contrib non-free" | tee -a /etc/apt/sources.list
RUN wget https://archive.raspbian.org/raspbian.public.key -O - | apt-key add -
RUN dpkg --add-architecture armhf
RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    libssl-dev:armhf \
    libsqlite3-dev:armhf

# http://snail-like.com/mediawiki/index.php/RPi_CPU_BCM2708_or_BCM2835
# BCM2708 is cpu family name
ENV RASPBERRY_PI_TOOLS_COMMIT_ID 5caa7046982f0539cf5380f94da04b31129ed521
RUN curl -sSL https://github.com/raspberrypi/tools/archive/$RASPBERRY_PI_TOOLS_COMMIT_ID.tar.gz \
  | tar -zxC /opt tools-$RASPBERRY_PI_TOOLS_COMMIT_ID/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64 --strip=2
ENV PATH /opt/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH $PATH:/root/.cargo/bin
RUN rustup target add arm-unknown-linux-gnueabihf
# cache
RUN cargo search cargo

WORKDIR /source

RUN mkdir -p /source/.cargo
# https://users.rust-lang.org/t/how-to-pass-cargo-linker-args/3163
# https://www.raspberrypi.org/forums/viewtopic.php?t=11629
# https://doc.rust-lang.org/cargo/reference/config.html
# https://github.com/aidanhs/rustnotes
RUN echo '\
[target.arm-unknown-linux-gnueabihf]\n\
linker = "arm-linux-gnueabihf-gcc"\n\
rustflags = [\n\
  "-C", "link-args=-Xlinker -v",\n\
  "-C", "link-arg=-march=armv6",\n\
  "-C", "link-arg=-mfpu=vfp",\n\
  "-C", "link-arg=-mfloat-abi=hard",\n\
  "-C", "link-arg=-Wl,--verbose",\n\
  "-C", "link-arg=-L/opt/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/arm-linux-gnueabihf/libc/usr/lib/arm-linux-gnueabihf/",\n\
  "-Z", "print-link-args",\n\
]\n\
' | tee -a /source/.cargo/config

ADD . /source
RUN cargo build -vv --target=arm-unknown-linux-gnueabihf -p server --release
RUN arm-linux-gnueabihf-readelf --arch-specific ./target/arm-unknown-linux-gnueabihf/release/server || echo "failed"
RUN arm-linux-gnueabihf-readelf -d ./target/arm-unknown-linux-gnueabihf/release/server || echo "failed"
RUN arm-linux-gnueabihf-ldd ./target/arm-unknown-linux-gnueabihf/release/server || echo "failed"
RUN arm-linux-gnueabihf-strip ./target/arm-unknown-linux-gnueabihf/release/server

FROM resin/rpi-raspbian
WORKDIR /opt
COPY --from=builder /source/target/arm-unknown-linux-gnueabihf/release/server /opt/server
CMD ["/opt/server"]
