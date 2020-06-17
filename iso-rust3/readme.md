# Rust における wasm-bindgen と wasm-pack と cargo-web と stdweb の違い

※ここでの作業はすべて nightly を前提としています

## 系統の違い

Rust から wasm へコンパイルするには emscripten を使う方法と、 wasm-bindgen を使う方法の２つあります

### emscripten 系統

* もともとは clang の吐いた LLVMIR から同等の JavaScript を出力できるツール
* 後に asm.js , wasm も出力できるようになった
* libc 相当の system call が JavaScript の世界でエミュレートできる(ex. FileSystem
* 既存の C/C++ で書かれたコードをブラウザで動くように変換するのが目的
* C/C++/Rust から JavaScript を呼び出すのが主な使い方
* 今日はこの話はしない

#### 主な登場人物

下に行くほど新しいツールです

* emscripten - C/C++ を LLVMIR を経由して asmjs やら wasm やらにコンパイルするためのツール - https://kripken.github.io/emscripten-site/docs/introducing_emscripten/about_emscripten.html
* Binaryen - 特に C/C++ を emscripten を使って wasm にコンパイルするためのツール - https://github.com/WebAssembly/binaryen
* emscripten-sys - Rust から emscripten のランタイムへのバインディングが入ったクレート - https://crates.io/crates/emscripten-sys
* stdweb - Rust から DOM を扱うためののインディングの入ったクレート - https://crates.io/crates/stdweb
* cargo-web - stdweb を使った Rust プロジェクトのビルドツール -  https://crates.io/crates/cargo-web

### wasm-bindgen 系統

* https://rustwasm.github.io
* Mozilla 肝いりの Rust を Web ブラウザで動かすためのツール
* emscripten 系列よりも新しいエコシステム
* JavaScript から Rust を呼び出すのが主な使い方
* 今日話すのはこれ

#### 主な登場人物

* wasm-bindgen - 基本的な型のなどが入ったクレート - 
* js-sys - Rust から JavaScript の値を生成するためのクレート
* web-sys - Rust から DOM とかを叩くためのクレート
* wasm-bindgen-futures - Rust の Future と JavaScript の Promise の型を相互変換するためのクレート
* wasm-bindgen-cli - wasm-bindgen や js-sys や web-sys クレートを使って生成した wasm ファイルに Rust と JS の FFI のランタイムを追加するビルドツール
* wasm-pack - wasm-bindgen を使った Rust コードを npm の package.json から呼ぶための設定ファイルを出力するビルドツール
* webpack - 2018 年現在最も普及してる JavaScript のビルドツール

## おおまかな歴史

* 2000年代初頭: Java アプレット、Slackwave Flash 全盛期
* 2005年: GoogleMap 、 Ajax の登場
* 2007年: MS が Silverlight を発表
* 2010年: emscripten が登場し C/C++ コードを JavaScript に変換できるようになる
* 2011年: Google が NaCl , PNaCl を発表
* 2013年: Mozilla が asm.js を発表
* 2015年: Mozilla や Google が asm.js をより一般化した wasm を発表
* 2017年: stdweb
* 2018年: wasm-bindgen

## wasm-bindgen-cli と wasm-pack の違い

* wasm-bindgen-cli は wasm と JavaScript のラッパーと TypeScript の型定義を出力してくれる
* wasm-pack は内部で wasm-bindgen-cli を使い、↑に加えて npm に公開するための package.json も出力する

### wasm-bindgen-cli の生成物

* https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html

これを

```toml:Cargo.toml
[package]
name = "iso-rust3"
version = "0.1.0"
authors = ["Legokichi Duckscallion <legokichi@gmail.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
futures = "0.1"
serde = "1.0"
serde_json = "1.0"
serde_derive = "1.0"
wasm-bindgen = { version = "0.2", features = [ "serde-serialize" ] }
wasm-bindgen-futures = "0.3"
js-sys = "0.3"

[dependencies.web-sys]
version = "0.3"
```

```rust:src/main.rs
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, World!");
}
```

こうして

```console
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
$ cargo build --target wasm32-unknown-unknown
$ wasm-bindgen target/wasm32-unknown-unknown/debug/iso_rust3.wasm --out-dir ./wasm
$ tree ./wasm
wasm
├── iso_rust3_bg.d.ts
├── iso_rust3_bg.wasm
├── iso_rust3.d.ts
└── iso_rust3.js
```

こうなる

```js:iso_rust3.js
/* tslint:disable */
import * as wasm from './iso_rust3_bg';

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_alert_d4ac2591f07b50f4(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    alert(varg0);
}
/**
* @returns {void}
*/
export function greet() {
    return wasm.greet();
}
```

↑は js-sys や web-sys を介して使った JavaScript 側の API を呼ぶためのランタイムコードがいろいろ追加されている。
ランタイムは JavaScript のオブジェクトを wasm の世界のメモリのヒープアロケーションをして、 Rust が読める形でデータを書き込み、そのポインタを Rust の関数に渡している。
より複雑なことをするとより多くのランタイムコードが出力される。

```typescript:iso_rust3.d.ts
/* tslint:disable */
export function greet(): void;
```

```typescript:iso_rust3_bg.d.ts
/* tslint:disable */
export const memory: WebAssembly.Memory;
export function greet(): void;
```

これらの生成物は JavaScript から ESModule を使ってこのように呼ぶことがでいる

```js
import("./wasm").then(iso_rust3 => {
    iso_rust3.greet("World!");
});
```


### wasm-pack の生成物

```console
$ rustup target add wasm32-unknown-unknown
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
$ wasm-pack build --dev
$ wasm-pack pack
$ pkg
├── iso-rust3-0.1.0.tgz
├── iso_rust3_bg.d.ts
├── iso_rust3_bg.wasm
├── iso_rust3.d.ts
├── iso_rust3.js
└── package.json
```

package.json 以外は同じ

```json
{
  "name": "iso-rust3",
  "collaborators": [
    "Legokichi Duckscallion <legokichi@gmail.com>"
  ],
  "version": "0.1.0",
  "files": [
    "iso_rust3_bg.wasm",
    "iso_rust3.js",
    "iso_rust3.d.ts"
  ],
  "module": "iso_rust3.js",
  "types": "iso_rust3.d.ts",
  "sideEffects": "false"
}
```

`wasm-pack pack` は iso-rust3-0.1.0.tgz を生成する。 `npm pack` 相当。


## おまけ: js-sys と web-sys

例えば WebAudioAPI の ScriptProcessor を使おうとすると以下のような複雑なコードを書くハメになる。

```rust
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use futures::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::*;
use web_sys::*;
use serde_derive::*;

#[wasm_bindgen]
pub fn main(){
    let window = window().unwrap();
    let document = window.document().unwrap();
    let media_devices = window.navigator().media_devices().unwrap();
    #[derive(Serialize)]
    struct Constraints {
        audio: bool,
        video: bool,
    }
    let constraints = JsValue::from_serde(&Constraints{audio: true, video: false}).unwrap();
    let prm = media_devices.get_user_media_with_constraints(<MediaStreamConstraints as JsCast>::unchecked_from_js_ref(&constraints)).unwrap();
    {
        let cb = Closure::wrap(Box::new(move |media_stream: JsValue|{
            console::log(&js_sys::Array::from(&JsValue::from_str("Hello, 💩!")));
            let src = Url::create_object_url_with_source(&media_stream.into()).unwrap();
            let audio = HtmlAudioElement::new_with_src(&src).unwrap();
            (audio.as_ref() as &HtmlMediaElement).set_autoplay(true);
            (audio.as_ref() as &HtmlMediaElement).set_controls(true);
            let cb = Closure::wrap(Box::new(move |ev|{
                console::log(&js_sys::Array::from(&JsValue::from_str("Hello, 💩!")));
                let actx = AudioContext::new().unwrap();
                let processor = (actx.as_ref() as &BaseAudioContext).create_script_processor().unwrap();
                let cb = Closure::wrap(Box::new(move |ev: Event|{
                    // let abuf = (ev.as_ref() as &AudioProcessingEvent).input_buffer().unwrap();
                    // やってられん！
                    console::log(&js_sys::Array::from(&JsValue::from_str("Hello, 💩!")));
                }) as Box<dyn FnMut(Event)>);
                processor.set_onaudioprocess(Some(cb.as_ref().unchecked_ref()));
                (processor.as_ref() as &AudioNode).connect_with_audio_node((actx.as_ref() as &BaseAudioContext).destination().as_ref());
                cb.forget();
            }) as Box<dyn FnMut(Event)>);
            (audio.as_ref() as &EventTarget).add_event_listener_with_callback("loadedmetadata", cb.as_ref().unchecked_ref()).unwrap();
            (document.body().unwrap().as_ref() as &Node).append_child(audio.as_ref()).unwrap();
            cb.forget();
        }) as Box<dyn FnMut(JsValue)>);
        let prm = prm.then(&cb);
        cb.forget();
    }
}
```


JSのAPI のイベントハンドラひとつ書くにしても

```
let cb = Closure::wrap(Box::new(move |ev|{
    // hogehoge
    cb.forget();
}) as Box<dyn FnMut(Event)>);
```

のようにクロージャのGCのタイミングも自分で指定せねばならず面倒くさい。
またランタイムは数百行に及ぶ（それでも emscripten のランタイムよりは遥かに少ないが）

