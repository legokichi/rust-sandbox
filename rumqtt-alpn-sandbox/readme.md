
```bash
cargo watch -x "run --release" 2>&1 | tee log.txt
aws iot describe-endpoint --endpoint-type iot:Data-ATS
```

```bash
cargo install --force --git https://github.com/rust-embedded/cross cross
git clone https://github.com/rust-embedded/cross
pushd cross
docker build -t arm-unknown-linux-musleabihf:latest -f docker/arm-unknown-linux-musleabihf/Dockerfile docker
popd
cross build --target arm-unknown-linux-musleabihf
readelf --arch-specific ./target/arm-unknown-linux-musleabihf/release/rumqtt-alpn-sandbox
```
