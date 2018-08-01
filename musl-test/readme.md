* https://github.com/rust-lang-nursery/embedded-wg

```sh
sudo apt install musl-tools
rustup target add x86_64-unknown-linux-musl
```

```sh
cargo +nightly build --target=x86_64-unknown-linux-musl --release
```

```sh
readelf -d target/x86_64-unknown-linux-musl/release/musl-test
readelf -A target/x86_64-unknown-linux-musl/release/musl-test
ldd target/x86_64-unknown-linux-musl/release/musl-test

cargo run
```

```sh
docker build --tag muslserver .
docker run -d \
  --name=muslserver \
  -e HOST_URL=0.0.0.0:3000 \
  -p 3000:3000 \
  muslserver
```