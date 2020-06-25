docker build --rm -t aws-rust-lambda-builder .
docker run --rm \
      -e CARGO_TARGET_DIR=/tmp/app/target \
      -v $(pwd)/:/tmp/app \
      -v $(pwd)/.cache/cargo/registry:/root/.cargo/registry \
      -v $(pwd)/.cache/cargo/git:/root/.cargo/git \
      -w /tmp/app \
      -u 0:$(id -u) \
      -t aws-rust-lambda-builder:latest \
      bash build.sh
