# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` hosts the CLI entry point; keep subcommands or helpers in additional modules under `src/` as the app grows.
- `.cargo/config.toml` pins the Android NDK toolchain (r27) and linker paths for the `aarch64-linux-android` target—adjust only when the NDK location changes.
- `Cargo.toml` is minimal; group new dependencies under `[dependencies]` and prefer workspace features only when multiple crates appear.

## Build, Test, and Development Commands
- `cargo build` compiles for the host toolchain; use before committing to catch syntax issues quickly.
- `cargo run` executes the CLI locally (x86_64) for rapid iteration.
- `cargo build --target aarch64-linux-android` emits the Termux binary into `target/aarch64-linux-android/<profile>/`; ensure the Android NDK is installed via `sdkmanager "ndk;27.0.12077973"`.
- `cargo fmt && cargo clippy --all-targets --all-features` enforces style and lints; run prior to review.

## Coding Style & Naming Conventions
- Follow Rust 2024 edition defaults: four-space indentation, snake_case for modules/functions, CamelCase for types, UPPER_SNAKE_CASE for constants.
- Use explicit modules (`mod subcommand;`) instead of huge files once functionality grows.
- Prefer descriptive binary messages (`println!`) over placeholder text when expanding the CLI.

## Testing Guidelines
- Use `cargo test` (unit + doc tests). Place unit tests in the same module under `#[cfg(test)]`; integration tests belong in `tests/` (create when needed).
- Name tests after expected behavior, e.g., `handles_missing_flags`.
- Track coverage informally for now; if behavior becomes critical, integrate `cargo llvm-cov`.

## Commit & Pull Request Guidelines
- Recent history shows `wip` messages; switch to imperative summaries such as “Add Termux linker config”.
- Reference issue IDs in the body when applicable and describe user-visible impact plus verification steps.
- For pull requests, include: purpose, testing output (`cargo build --target aarch64-linux-android`), and any deployment notes (e.g., `adb push` paths). Attach screenshots only if CLI output is visually relevant.

## Agent Tips
- Verify `$ANDROID_HOME` and `$ANDROID_NDK_HOME` when the build fails—missing environment variables usually indicate a moved SDK.
- When updating toolchain versions, bump paths in `.cargo/config.toml` and document the change here to keep future agents aligned.
