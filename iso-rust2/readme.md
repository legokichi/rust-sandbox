* https://github.com/rustwasm/wasm-bindgen
* https://rustwasm.github.io/wasm-pack/commands/build.html
* wasm-bindgen tutorial - https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html
* wasm-pack - https://rustwasm.github.io/book/game-of-life/setup.html
* wasm-bindgen - https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/
* js-sys - https://rustwasm.github.io/wasm-bindgen/api/js_sys/
* web_sys::Window - https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.alert


```bash
cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/iso_rust2.wasm --out-dir .
npm run serve
```

```bash
cargo watch -x "build --target wasm32-unknown-unknown" &
npm run serve &
open localhost:8080
wasm-bindgen target/wasm32-unknown-unknown/debug/iso_rust2.wasm --out-dir .
```