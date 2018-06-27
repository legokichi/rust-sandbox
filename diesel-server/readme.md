* https://github.com/diesel-rs/diesel/tree/master/diesel_cli

```sh
sudo apt install libsqlite3-dev
cargo install diesel_cli -f --no-default-features --features "postgres sqlite"
cargo install-update -a 
```

* https://github.com/diesel-rs/diesel/tree/master/examples/sqlite/getting_started_step_3

```sh
diesel migration generate blog
```

```sh
diesel database reset
diesel migration run
```

`diesel.toml` + `diesel migration run` = `diesel print-schema | tee db/src/schema.rs`