FROM debian:jessie as builder
ENV DEBIAN_FRONTEND noninteractive
ENV PKG_CONFIG_ALLOW_CROSS 1


RUN echo "" > /etc/apt/sources.list
RUN echo "deb [arch=armhf] http://mirrordirector.raspbian.org/raspbian/ jessie main contrib non-free rpi" | tee -a /etc/apt/sources.list
RUN echo "deb http://httpredir.debian.org/debian jessie main" | tee -a /etc/apt/sources.list
RUN echo "deb http://httpredir.debian.org/debian jessie-updates main" | tee -a /etc/apt/sources.list
RUN echo "deb http://security.debian.org jessie/updates main" | tee -a /etc/apt/sources.list
RUN dpkg --add-architecture armhf
RUN apt-get update -y
RUN apt-key adv --recv-keys --keyserver keys.gnupg.net 9165938D90FDDD2E
RUN apt-get install --force-yes -y --no-install-recommends \
    binutils curl wget zip git vim \
    build-essential \
    ca-certificates \
    file \
    pkg-config \
    curl \
    libssl-dev \
    libssl-dev:armhf \
    libsqlite3-dev:armhf

# http://snail-like.com/mediawiki/index.php/RPi_CPU_BCM2708_or_BCM2835
# BCM2708 is cpu family name
ENV RASPBERRY_PI_TOOLS_COMMIT_ID 5caa7046982f0539cf5380f94da04b31129ed521
RUN curl -sSL https://github.com/raspberrypi/tools/archive/$RASPBERRY_PI_TOOLS_COMMIT_ID.tar.gz \
  | tar -zxC /opt tools-$RASPBERRY_PI_TOOLS_COMMIT_ID/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64 --strip=2
ENV CC_DIR /opt/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin
ENV PATH $CC_DIR:$PATH


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
RUN echo '[target.arm-unknown-linux-gnueabihf]' | tee -a /source/.cargo/config
RUN echo 'linker = "arm-linux-gnueabihf-gcc"' | tee -a /source/.cargo/config
RUN echo 'rustflags = [' | tee -a /source/.cargo/config
RUN echo '  "-C", "link-args=-Xlinker -v",' | tee -a /source/.cargo/config
RUN echo '  "-C", "link-arg=-march=armv6",' | tee -a /source/.cargo/config
RUN echo '  "-C", "link-arg=-mfpu=vfp",' | tee -a /source/.cargo/config
RUN echo '  "-C", "link-arg=-mfloat-abi=hard",' | tee -a /source/.cargo/config
RUN echo '  "-C", "link-arg=-isystem/usr/include/arm-linux-gnueabihf",' | tee -a /source/.cargo/config
RUN echo '  "-C", "link-arg=-isystem/usr/include",' | tee -a /source/.cargo/config
RUN echo '  "-C", "link-arg=-L/usr/lib/arm-linux-gnueabihf",' | tee -a /source/.cargo/config
RUN echo '  "-C", "link-arg=-L/usr/lib",' | tee -a /source/.cargo/config
RUN echo '  "-C", "link-arg=-Wl,--verbose",' | tee -a /source/.cargo/config
RUN echo '  "-Z", "print-link-args",' | tee -a /source/.cargo/config
RUN echo ']' | tee -a /source/.cargo/config

ADD . /source
RUN cargo build -v --target=arm-unknown-linux-gnueabihf -p server
RUN readelf --arch-specific ./target/arm-unknown-linux-gnueabihf/debug/server
RUN readelf -d ./target/arm-unknown-linux-gnueabihf/debug/server

# FROM arm32v6/alpine:latest
# # RUN apk add --update --no-cache \
# #     sqlite-libs \
# #     sqlite-dev \
# #     sqlite
# WORKDIR /opt
# COPY --from=builder /source/target/arm-unknown-linux-gnueabihf/release/server server
# EXPOSE 3000
# CMD ["/opt/server"]