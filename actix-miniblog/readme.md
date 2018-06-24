



```sh
sudo apt -y install gcc-6-arm-linux-gnueabihf
cat <<'EOF' > .cargo/config
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc-6"
EOF
rustup toolchain install stable-arm-unknown-linux-gnueabihf
rustup target add arm-unknown-linux-gnueabihf
env CC_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-6  \
    AR_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc-ar-6
    cargo build -v --target=arm-unknown-linux-gnueabihf --release
scp -i ~/foo./target/arm-unknown-linux-gnueabihf/release/server pi@pigi.local:/home/pi/ 
```

```sh
docker run -it --rm -v $(pwd):/source dlecan/rust-crosscompiler-arm:stable \
  env CC=arm-linux-gnueabihf-gcc \
      CC_arm_unknown_linux_gnu=arm-linux-gnueabihf-gcc-with-link-search \
    cargo build --release
```

```sh
cargo watch -x 'run -p -- foo bar'
```

```
docker run -p 8086:8086 \
      -v influxdb:/var/lib/influxdb \
      influxdb
```

https://github.com/djc/askama/tree/master/testing


```
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

```sh
cargo build --target arm-unknown-linux-musleabihf --release 
```

```
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:arm-musleabihf cargo build --release
```