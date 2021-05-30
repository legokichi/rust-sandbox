wasm-pack build --target=nodejs
wasm-pack build --target=nodejs --release
twiggy top pkg/model_crate_bg.wasm
wasm-pack test --node


* https://docs.rs/crate/web-sys/0.3.46
* https://docs.rs/crate/js-sys/0.3.46
* https://docs.rs/console_log/0.2.0/console_log/
* https://rustwasm.github.io/docs/book/
* https://rustwasm.github.io/wasm-bindgen/
* https://rustwasm.github.io/wasm-pack/book/
* https://github.com/rustwasm/awesome-rust-and-webassembly



# "parcel-plugin-wasm.rs": "^1.3.0",

```js
const fs = require('fs')

const wasm_loader_path = __dirname + '/wasm-loader.js'
if (!fs.existsSync(wasm_loader_path)) {
  fs.writeFileSync(wasm_loader_path, '')
}

module.exports = function (bundler) {
  bundler.addBundleLoader('wasm', require.resolve('./wasm-loader'))
  bundler.addAssetType('toml', require.resolve('./WASMbindgenAsset'))
  bundler.addAssetType('rs', require.resolve('./WASMbindgenAsset'))
}
```

# "parcel-plugin-wasm-pack": "^6.0.1"

```js
module.exports = function (bundler) {
  delete bundler.bundleLoaders.wasm;

  bundler.addAssetType('toml', require.resolve('./src/WasmPackAsset'));
  bundler.addAssetType('rs', require.resolve('./src/WasmPackAsset'));

  bundler.addPackager('js', require.resolve('./src/WasmPackPackager'));
};
```

# wasm_rust_crate.js

## wasm-build --target bundler

```ts
import * as wasm from "./wasm_rust_crate_bg.wasm";
export * from "./wasm_rust_crate_bg.js";
```

### pkg/wasm_rust_crate.js --target node

```ts
import * as A from "pkg";
A.start();
```

#### parcel

```
✨  Built in 737ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     37ms
dist/wasm_rust_crate.js                  18.91 KB    635ms
(node:278050) UnhandledPromiseRejectionWarning: TypeError: WebAssembly.instantiate(): Imports argument must be present and must be an object
```

#### "parcel-plugin-wasm.rs": "^1.3.0"

```
✨  Built in 867ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     33ms
dist/wasm_rust_crate.js                  18.91 KB    670ms
(node:278429) UnhandledPromiseRejectionWarning: TypeError: WebAssembly.instantiate(): Imports argument must be present and must be an object
```

#### "parcel-plugin-wasm-pack": "^6.0.1"

```
✨  Built in 833ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     30ms
dist/wasm_rust_crate.js                  18.91 KB    710ms
(node:279108) UnhandledPromiseRejectionWarning: TypeError: WebAssembly.instantiate(): Imports argument must be present and must be an object
```


#### both

```
✨  Built in 744ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     38ms
dist/wasm_rust_crate.js                  18.91 KB    636ms
(node:279304) UnhandledPromiseRejectionWarning: TypeError: WebAssembly.instantiate(): Imports argument must be present and must be an object
```

### index.mjs --target node

```ts
import * as A from "pkg";
A.start();
```

#### parcel

```
✨  Built in 737ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     23ms
dist/index.js                            19.14 KB    636ms
(node:281014) UnhandledPromiseRejectionWarning: TypeError: WebAssembly.instantiate(): Imports argument must be present and must be an object
```

#### "parcel-plugin-wasm.rs": "^1.3.0"

```
✨  Built in 740ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     23ms
dist/index.js                            18.72 KB    614ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116
    throw error;
    ^

TypeError: bundleLoader is not a function
```

#### "parcel-plugin-wasm-pack": "^6.0.1"

```
✨  Built in 682ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     54ms
dist/index.js                            15.92 KB    432ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116
    throw error;
    ^

TypeError: wasm.start is not a function
```

#### both

```
✨  Built in 652ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     39ms
dist/index.js                            15.92 KB    393ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116
    throw error;
    ^

TypeError: wasm.start is not a function
```


### index.js --target node

```ts
const A = require("pkg");
A.start();
```

#### parcel

```
✨  Built in 669ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     19ms
dist/index.js                            19.05 KB    554ms
(node:283013) UnhandledPromiseRejectionWarning: TypeError: WebAssembly.instantiate(): Imports argument must be present and must be an object
```

#### "parcel-plugin-wasm-pack": "^6.0.1"

```
✨  Built in 661ms.

dist/wasm_rust_crate_bg.7dcb1f9e.wasm    51.66 KB     42ms
dist/index.js                            15.79 KB    393ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116
    throw error;
    ^

TypeError: wasm.start is not a function
```


## wasm-build --target nodejs

```ts
const { sleep } = require(String.raw`./snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js`);
const { TextDecoder, TextEncoder } = require(String.raw`util`);

~~~~~~~~~~~~~~~~~~

const path = require('path').join(__dirname, 'wasm_rust_crate_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;
```

### pkg/wasm_rust_crate.js --target node

#### parcel

```
✨  Built in 409ms.

dist/wasm_rust_crate.js    11.1 KB    195ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/wasm_rust_crate.js:116
    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
Require stack:
- /home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/wasm_rust_crate.js
    at Function.Modul
```

#### "parcel-plugin-wasm.rs": "^1.3.0"

```
✨  Built in 404ms.

dist/wasm_rust_crate.js    11.1 KB    193ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/wasm_rust_crate.js:116
    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
```

#### "parcel-plugin-wasm-pack": "^6.0.1"

```
✨  Built in 400ms.

dist/wasm_rust_crate.js    11.1 KB    169ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/wasm_rust_crate.js:116
    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
```

#### both

```
✨  Built in 391ms.

dist/wasm_rust_crate.js    11.1 KB    180ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/wasm_rust_crate.js:116
    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
```

### index.mjs --target node

```ts
import * as A from "pkg";
A.start();
```

#### parcel

```
✨  Built in 401ms.

dist/index.js    11.66 KB    139ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116
    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
```

#### "parcel-plugin-wasm.rs": "^1.3.0"

```
✨  Built in 384ms.

dist/index.js    11.66 KB    151ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116
    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
```

#### "parcel-plugin-wasm-pack": "^6.0.1"

```
✨  Built in 405ms.

dist/index.js    11.66 KB    162ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116
    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
```

#### both

```
✨  Built in 401ms.

dist/index.js    11.66 KB    177ms
internal/modules/cjs/loader.js:883
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
```

### index.js --target node

```ts
const A = require("pkg");
A.start();
```

#### parcel

```
✨  Built in 363ms.

dist/index.js    11.57 KB    108ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116
    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
```

#### "parcel-plugin-wasm-pack": "^6.0.1"

```
✨  Built in 335ms.

dist/index.js    11.53 KB    105ms
/home/legokichi/Github/rust-snipets/wasm-pack-sandbox/wasm-rust-crate/dist/index.js:116
    throw error;
    ^

Error: Cannot find module './snippets/wasm-rust-crate-d884aa388a611dd4/inline0.js'
```

