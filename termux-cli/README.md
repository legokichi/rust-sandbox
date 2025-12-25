# termux-cli

A minimal Rust-based “Hello, world!” CLI meant to demonstrate cross-compiling to Termux on Android (`aarch64-linux-android`). It doubles as a sandbox for experimenting with toolchain configuration, custom subcommands, and future automation agents.

## Project Layout
- `Cargo.toml` — crate metadata and dependency list.
- `.cargo/config.toml` — target-specific linker and archiver paths pointing to Android NDK r27.
- `src/main.rs` — CLI entry point; expand with modules in `src/` as the app grows.

## Build & Run (Host)
```bash
cargo build
cargo run
```
Use the host build for quick iterations on x86_64 Linux.

## Cross-Compile for Termux (aarch64)
1. (Optional) Install cargo-cross for easier cross-compilation:
   ```bash
   cargo install cross
   ```
2. Install Android NDK 27 via the SDK manager:
   ```bash
   sdkmanager "ndk;27.0.12077973"
   ```
3. Ensure the paths in `.cargo/config.toml` match your `$HOME/android-sdk` installation.
4. Build the Android target (using cross if installed, otherwise cargo):
   ```bash
   cross build --target aarch64-linux-android
   # or
   cargo build --target aarch64-linux-android
   ```
5. Push the binary to a device (example path):
   ```bash
   adb push target/aarch64-linux-android/debug/termux-cli /data/data/com.termux/files/home/bin/
   ```

## Development Tips
- Keep commits small and descriptive (e.g., “Add Termux linker config”).
- Run `cargo fmt` and `cargo clippy --all-targets --all-features` before opening a PR.
- Add new tests with `cargo test`; place integration tests inside `tests/` when the CLI gains behavior beyond the greeting.

## Roadmap Ideas
- Add argument parsing (e.g., `clap`) for multiple subcommands.
- Package nightly/release artifacts for automated Termux installs.
- Integrate CI jobs that exercise both host and `aarch64-linux-android` targets.
