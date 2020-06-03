# MIRi

* https://github.com/rust-lang/miri
* mid-level Intermediate Representation Interpriter(中間表現インタプリタ)
* rustc の constexpr (ぽいやつ)のためのインタプリタ
* 「正式な検証ツールではない」が参照エラーや未定義動作を検出できる（なんで？）


```
export RUSTC_WRAPPER=""
cargo clean
cargo miri test
```

# シンボリック実行
## cargo-klee

* https://gitlab.henriktjader.com/pln/cargo-klee
* KLEE - シンボリック実行仮想マシン
* シンボリック実行 - https://speakerdeck.com/katc/girls-meets-symbolic-execution-assertion-2-automated-exploit-generation?slide=9
* コードの実行パスを解析してそのパスに入る条件を調べる
* パスが爆発するので巨大なプログラムは実質解析不能
* `cargo klee --bin foo --release`
* https://gitlab.henriktjader.com/pln/cargo-klee/-/blob/master/klee-examples/src/foo.rs
* https://gitlab.henriktjader.com/pln/cargo-klee/-/blob/master/klee-examples/Cargo.toml
* no_std no_main no_mangle にしたり --bin しか解析できなかったり割と面倒そう

## haybale

* https://github.com/PLSysSec/haybale
* Rust で書かれた LLVM IR 向け(つまり c, c++, rust 向け)のシンボリック実行
* シンボリック実行 -> 関数｜プログラムの動作を数学的に厳密に推論する
* ある関数が 0 を返しうるか、パニックしうるかどうかを調査できる
* その他 llvm - https://gist.github.com/MattPD/00573ee14bf85ccac6bed3c0678ddbef#llvm---verification

# 自動検証ツール
## MIRAI

* https://github.com/facebookexperimental/MIRAI
* facebook の検証のための mir 実行エンジンのよう
* 汚染分析 - 信用できない関数を使っている関数は信用できない
* プロトコル（状態遷移）検証
* https://lib.rs/crates/mirai-annotations
* https://crates.io/crates/mirai_annotations
* https://github.com/facebookexperimental/MIRAI/tree/master/annotations

```
cd MIRAI
./setup.sh
RUSTFLAGS='-Clink-arg=-L./binaries -Clink-arg=-lstdc++' cargo install --path ./checker -f
export RUSTC_WRAPPER=mirai cargo build
cp rust-toolchain /path/to/your_project_dir
```

## Crux-mir(mir-verifier)

* https://github.com/GaloisInc/mir-verifier
* MIR 静的シミュレータ
* 自動検証ツール

# 自動アクティブ検証ツール
## Prusti

* https://github.com/viperproject/prusti-dev
* Viper中間検証言語 にコンパイルして検証 - https://www.pm.inf.ethz.ch/research/viper.html
* `unreachable!();` は検出できるけど `println!()` で prusti が unreachable に突入する
* vscode の拡張がある - https://marketplace.visualstudio.com/items?itemName=viper-admin.prusti-assistant

```
rustup install nightly-2018-06-27
```

# 演繹的検証ツール
## RustBelt
* Iris と Coq で rust コードを検証


# 契約プログラミング
* https://gitlab.com/karroffel/contracts
    * 関数の pre: 事前条件 post: 事後条件 invariant: 不変条件 を注釈できる
    * 実行時アサーションを derive macro で書ける
    * MIRAI 向け
    * https://lib.rs/crates/contracts
    * https://docs.rs/contracts/0.4.0/contracts/
* https://github.com/viperproject/rust-contracts
    * viperproject 公式
    * requires: 事前 ensures: 事後 invariant: 不変
* https://docs.rs/verifier/0.1.0/verifier/
    * https://github.com/soarlab/rust-benchmarks
    * mirai 向け

# fuzzing

## cargo fuzz

```
cargo fuzz run fuzz_target_1
```

