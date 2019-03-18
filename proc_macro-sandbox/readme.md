* https://doc.rust-lang.org/beta/book/ch19-06-macros.html
* https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html
* https://doc.rust-lang.org/proc_macro/
* https://docs.rs/proc-macro2/0.4.27/proc_macro2/
* https://github.com/alexcrichton/proc-macro2


prop_macro は std の型。

```toml
[lib]
proc-macro = true
```

しないと使えない。

proc_macro2 は proc_macro 互換のどこでも使える外部クレート。
proc_macro の型を proc_macro2 に変換して使うことで proc_macro をどこでも（テスト,build.rs,etc...)使えるようにできる。