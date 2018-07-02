## init

* https://github.com/diesel-rs/diesel/tree/master/diesel_cli

```sh
sudo apt install libsqlite3-dev
cargo install diesel_cli -f --no-default-features --features "postgres sqlite"
cargo install-update -a 
```

## migration

* https://github.com/diesel-rs/diesel/tree/master/examples/sqlite/getting_started_step_3

```sh
diesel migration generate blog
```

```sh
diesel setup
//diesel database reset
diesel migration run
```

`diesel.toml` + `diesel migration run` = `diesel print-schema | tee db/src/schema.rs`

## run as docker

```sh
docker run -ti --rm armhf/alpine bash server
```


## fmt

```sh
rustup component add rustfmt-preview --toolchain nightly
cargo +nightly fmt
```

## clippy

```sh
rustup update nightly
cargo +nightly install --force clippy
cargo +nightly clippy
```

## cross


```sh
docker run -it --rm -v $(pwd):/source dlecan/rust-crosscompiler-arm:stable \
  env CC=arm-linux-gnueabihf-gcc \
  cargo build --release
      CC_arm_unknown_linux_gnu=arm-linux-gnueabihf-gcc-with-link-search \
      CC_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-6 \
      AR_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-ar-6 \
      cargo build --release

docker run -it --rm -v $(pwd):/source dlecan/rust-crosscompiler-arm:stable \
  env env CC=arm-linux-gnueabihf-gcc \
      CC_arm_unknown_linux_gnu=arm-linux-gnueabihf-gcc-with-link-search \
  cargo build --release
```

see

* https://github.com/dlecan/rust-crosscompiler-arm/blob/master/arm/Dockerfile
* https://github.com/dlecan/rust-crosscompiler-arm/tree/master/arm/include

```sh
docker run -ti debian:jessie bash

rustup toolchain install nightly-arm-unknown-linux-gnueabihf
rustup target add arm-unknown-linux-gnueabihf

sudo dpkg --add-architecture armhf
sudo apt update
sudo apt install libsqlite3-dev:armhf
cargo build --target=arm-unknown-linux-gnueabihf
sudo dpkg --remove-architecture armhf
suod apt purge ".*:armhf"
```

