* https://github.com/rustwasm/wasm-bindgen
* https://rustwasm.github.io/wasm-pack/commands/build.html
* wasm-bindgen tutorial - https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html
* wasm-pack - https://rustwasm.github.io/book/game-of-life/setup.html
* wasm-bindgen - https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/
* js-sys - https://rustwasm.github.io/wasm-bindgen/api/js_sys/
* web_sys::Window - https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.alert
* js_sys - 
```bash
cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/js_hello_world.wasm --out-dir .
npm run serve
```

```bash
cargo init
wasm-pack init
cargo watch -x "build --target wasm32-unknown-unknown"
```