FROM amazonlinux:2018.03.0.20200318.1

ENV RUST_VERSION nightly
ENV OS_VERSION 0.18.0
RUN yum install -y gcc gcc-c++ make openssl openssl-devel zip unzip git \
    && rm -rf /var/cache/yum/* \
    && yum clean all
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain "${RUST_VERSION}"
ENV PATH $PATH:/root/.cargo/bin
RUN rustup component add rustfmt clippy

