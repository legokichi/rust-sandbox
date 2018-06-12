
```sh
rustup toolchain install nightly
rustup default nightly
cargo new asio
cargo run -- --port 3000
cargo run --release -- --port 3000
pmap -x [pid]
```



## developping tools

### rust-toolchain
* https://github.com/rust-lang-nursery/rustup.rs#the-toolchain-file

### cargo doc

```sh
cargo doc --open
```

### cargo-clippy (linter)
* https://rust-lang-nursery.github.io/rust-clippy/master/index.html

```sh
rustup update nightly
cargo +nightly install --force clippy
cargo +nightly clippy
```

### cargo-watch

```sh
cargo install cargo-watch
```

```sh
cargo watch -x test
cargo watch -x 'run -- --some-arg'
```

### cargo-tree

```sh
cargo install cargo-tree
cargo tree
```

### stripe
* `cargo rustc --release -- -C link-args=-Wl,-S`
* https://qiita.com/hhatto/items/75d12de5a39ee37c5ddb
* https://jamesmunns.com/update/2018/04/01/tinyrocket.html


## testing
* https://doc.rust-lang.org/book/second-edition/ch11-00-testing.html

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

```sh
cargo test
```

## debugging tools

### env_logger
* https://qiita.com/rejasupotaro/items/e45fe64623ac7462e2a9

```rust
extern crate env_logger;
extern crate log;

fn main() {
    env_logger::init();
    trace!(...);
    debug!(...);
    info!(...);
    warn!(...);
    error!(...);
}
```

RUST_LOG つけたときだけ出力

```sh
env RUST_LOG=trace ./target/debug/asio
```

### RUST_BACKTRACE

```
env RUST_BACKTRACE=1 ./target/debug/asio
```

## 変数の型名を取得
* https://qiita.com/ubnt_intrepid/items/4995c51d1271cc9529f4
`the_answer_of_everything` 帰り値の型が知りたいとき
```rust
fn the_answer_of_everything() -> i32 { 42 }

fn main() {
  let _ :() = the_answer_of_everything();
}
```

変数に間違った型をつけてコンパイルエラーを起こす

```
error: mismatched types [--explain E0308]
 --> <anon>:4:15
  |>
4 |>   let _ :() = the_answer_of_everything();
  |>               ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found i32
note: expected type `()`
note:    found type `i32`
```

### rust-xxdb
* https://michaelwoerister.github.io/2015/03/27/rust-xxdb.html
* lldb <-> gdb 対応表 - http://lldb.llvm.org/lldb-gdb.html

```sh
rust-gdb -statistics --args ./your-program --prog-arg1 --prog-arg2 foo bar
rust-lldb  --  ./your-program --prog-arg1 --prog-arg2 foo bar
```

#### rust-gdb
#### rust-lldb

lldb を起動する

```console
rust-lldb ./target/debug/asio
```

ブレークポイント

```
(lldb) breakpoint set --name access
(lldb) breakpoint set --name xxx::http::access
(lldb) breakpoint set --file main.rs --line 1
(lldb) breakpoint list
```

```
(lldb) run
(lldb) step
(lldb) next
```

```
(lldb) frame variable x
```

#### vscode-lldb
* http://asquera.de/blog/2017-03-03/setting-up-a-rust-devenv/
* http://lustysociety.org/programming/rust_language/rust-language-linux.html
* https://qiita.com/kat_out/items/cf89a8dffb4e0629948a

```sh
code --install-extension rust-lang.rust
code --install-extension vadimcn.vscode-lldb
```

##### Borrow visualizer for the Rust Language Service

* https://internals.rust-lang.org/t/borrow-visualizer-for-the-rust-language-service/4187/36
* https://github.com/Nashenas88/borrow_visualizer_prototype

#### gdbgui

* https://github.com/cs01/gdbgui

```sh
sudo pip3 install gdbgui --upgrade
```
```sh
gdbgui \
 --args "./target/debug/asio" \
 --lldb \
 --port 8080
```

`--rr`

##### rr

* http://rr-project.org
* record and replay execution

### cargo asm

* https://keens.github.io/blog/2018/04/04/cargo_asmderustnomemorimawarisaitekikawochekku/

```rust
// some_crate
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

```sh
cargo asm main::main
cargo llvm-ir main::main
```

### cargo-expand
* https://github.com/dtolnay/cargo-expand

```sh
cargo install cargo-expand
cargo expand
```

### cargo-profiler

* https://github.com/kernelmachine/cargo-profiler

```sh
sudo apt-get install valgrind
cargo install cargo-profiler
```

```sh
cargo profiler callgrind
cargo profiler cachegrind --release
```

### Thread Profiler

* https://github.com/glennw/thread_profiler

### cargo binutils

* `cargo nm -- a.out`
* `cargo objdump --  -disassemble -no-show-raw-insn a.out` 
* `cargo objdump -- -s -b binary a.out | head`

### readelf
共有ライブラリについて調べたいとき

* `env | grep LD` - パスを調べる
* `readelf -a hogehoge.so | grep SONAME` - SONAME を調べる
* `find build/ -name '*so' | xargs ldd | less` - 依存関係を調べる
* `LD_DEBUG=all <コマンドを実行>` - 共有ライブラリの読み込みに関して大量のデバッグログが吐かれる
* `find ./ -name "*.so*" -print | xargs readelf -d` - 共有ライブラリのシンボルテーブルを探す
* `readelf  -h a.out`
* `readelf  -l a.out`
* `readelf -s`


