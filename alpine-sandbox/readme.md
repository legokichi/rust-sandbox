
```bash
rustup target list | grep musl
rustup target add x86_64-unknown-linux-musl
RUSTFLAGS="-C target-feature=+crt-static" cargo run --target x86_64-unknown-linux-musl --release
```

```console
$ RUSTFLAGS="-C target-feature=+crt-static" cargo run --target x86_64-unknown-linux-musl --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/x86_64-unknown-linux-musl/release/alpine-sandbox`
Hello, world!
$ readelf -d target/x86_64-unknown-linux-musl/release/alpine-sandbox

このファイルには動的セクションがありません。
```




* https://github.com/messense/rust-musl-cross
* https://users.rust-lang.org/t/statically-linking-parts-of-a-shared-library/16171




## openssl with glibc

```bash
cargo run --release --features=openssl

```

## rustls with musl static linking

```bash
RUSTFLAGS="-C target-feature=+crt-static" cargo run --target x86_64-unknown-linux-musl --release --features=rustls
```