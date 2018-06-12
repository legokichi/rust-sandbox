SHELL := /bin/bash

all:
	cargo build --release

wasm32:
	export EMMAKEN_CFLAGS="-s USE_PTHREADS=1"
	cargo build --target=wasm32-unknown-emscripten
	find target/wasm32-unknown-emscripten/debug/deps -type f ! -name "*.asm.js" -name "*.js" | xargs -I {} cp {} site/site.js
	find target/wasm32-unknown-emscripten/debug/deps -type f -name "*.wasm" | xargs -I {} cp {} site/site.wasm

asmjs:
	export EMMAKEN_CFLAGS="-s USE_PTHREADS=1 -s ERROR_ON_UNDEFINED_SYMBOLS=1"
	cargo build --target=asmjs-unknown-emscripten --keep-stage 0
	find target/asmjs-unknown-emscripten/debug/ -maxdepth 1 -type f -name "*.js" | xargs -I {} cp {} site/site.js