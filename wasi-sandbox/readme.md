# wasmtime

install runtimes

```bash
cd ..
git clone --recurse-submodules https://github.com/CraneStation/wasmtime.git
cd wasmtime
cargo build --release
cd target/release
export PATH=$(pwd):$PATH
```

## wasm32-unknown-wasi

build project

```bash
rustup target add wasm32-unknown-wasi --toolchain nightly
cargo +nightly build --target wasm32-unknown-wasi --release
wasmtime target/wasm32-unknown-wasi/release/wasi-sandbox.wasm
```

# emscripten

install runtimes

```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install sdk-tag-1.38.28-64bit
./emsdk activate sdk-tag-1.38.28-64bit
source ./emsdk_env.sh
```

## wasm32-unknown-emscripten

```bash
rustup target add wasm32-unknown-emscripten --toolchain nightly
cargo +nightly build --target wasm32-unknown-emscripten
node target/wasm32-unknown-emscripten/debug/wasi-sandbox.js
```

* (`-s USE_PTHREADS=1 -s WASM=1`) - https://github.com/emscripten-core/emscripten/wiki/Pthreads-with-WebAssembly

## asmjs-unknown-emscripten

* https://emscripten.org/docs/porting/pthreads.html

```bash
rustup target add asmjs-unknown-emscripten --toolchain nightly
cargo +nightly build --target asmjs-unknown-emscripten
node target/asmjs-unknown-emscripten/debug/wasi-sandbox.js
```
