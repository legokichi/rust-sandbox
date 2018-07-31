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

RUN echo '#!/bin/bash' | tee -a /usr/local/sbin/arm-linux-gnueabihf-gcc-with-link-search
RUN echo "arm-linux-gnueabihf-gcc -mcpu=arm1176jzf-s -march=armv6 -mfpu=vfp -mfloat-abi=hard -isystem/usr/include/arm-linux-gnueabihf -isystem/usr/include -L/usr/lib/arm-linux-gnueabihf -L/usr/lib \$@" | tee -a /usr/local/sbin/arm-linux-gnueabihf-gcc-with-link-search
RUN chmod +x /usr/local/sbin/arm-linux-gnueabihf-gcc-with-link-search

RUN echo '#!/bin/bash' | tee /usr/local/sbin/arm-linux-gnueabihf-g++-with-link-search
RUN echo "arm-linux-gnueabihf-g++ -mcpu=arm1176jzf-s -march=armv6 -mfpu=vfp -mfloat-abi=hard -isystem/usr/include/arm-linux-gnueabihf -isystem/usr/include -L/usr/lib/arm-linux-gnueabihf -L/usr/lib \$@" | tee /usr/local/sbin/arm-linux-gnueabihf-g++-with-link-search
RUN chmod +x /usr/local/sbin/arm-linux-gnueabihf-g++-with-link-search

ENV CC_DIR /opt/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin
ENV REAL_CC $CC_DIR/arm-linux-gnueabihf-gcc
ENV CC arm-linux-gnueabihf-gcc-with-link-search
# ENV CC_arm_unknown_linux_gnueabihf arm-linux-gnueabihf-gcc-with-link-search
ENV CXX arm-linux-gnueabihf-g++-with-link-search
# ENV CXX_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++-with-link-search
ENV AR arm-linux-gnueabihf-gcc-ar
ENV OBJCOPY arm-linux-gnueabihf-objcopy

RUN ls -la /usr/lib/arm-linux-gnueabihf
RUN ls -la /opt/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin

WORKDIR /source

RUN mkdir -p /source/.cargo
# https://users.rust-lang.org/t/how-to-pass-cargo-linker-args/3163
# https://www.raspberrypi.org/forums/viewtopic.php?t=11629
RUN echo "[target.arm-unknown-linux-gnueabihf]" | tee -a /source/.cargo/config
RUN echo "linker = \"arm-linux-gnueabihf-gcc-with-link-search\"" | tee -a /source/.cargo/config

ADD . /source
RUN cargo build --target=arm-unknown-linux-gnueabihf -p server
RUN readelf --arch-specific ./target/arm-unknown-linux-gnueabihf/release/server


# FROM arm32v6/alpine:latest
# # RUN apk add --update --no-cache \
# #     sqlite-libs \
# #     sqlite-dev \
# #     sqlite
# WORKDIR /opt
# COPY --from=builder /source/target/arm-unknown-linux-gnueabihf/release/server server
# EXPOSE 3000
# CMD ["/opt/server"]