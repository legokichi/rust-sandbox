# Repository Guidelines

## Project Structure & Module Organization
- `src/bin/` holds binary examples like `debug.rs`, `blink.rs`, and `button.rs`.
- `build.rs` configures build-time behavior for the embedded target.
- `.cargo/config.toml` pins the target (`thumbv8m.main-none-eabihf`) and runner (`probe-rs`).
- `memory.x` defines the linker memory map for RP2350-class MCUs.
- `README.md` documents hardware connection checks.

## Build, Test, and Development Commands
- `cargo build` builds the firmware for the configured target in `.cargo/config.toml`.
- `cargo run` flashes and runs on a connected board via `probe-rs run --chip RP2350`.
- `cargo check` is a fast sanity check without producing a binary.

## Coding Style & Naming Conventions
- Rust standard conventions: `snake_case` for functions/variables, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Indentation: 4 spaces; keep lines concise where possible.
- Prefer explicit imports in `use` blocks; keep embedded-specific code `no_std`-friendly.
- If formatting is needed, use `cargo fmt` (rustfmt defaults are fine).

## Testing Guidelines
- There are no automated tests in this repository today.
- If you add host-testable logic, isolate it behind `#[cfg(test)]` modules and run `cargo test` (may require `std`).

## Commit & Pull Request Guidelines
- Existing history uses short, lowercase messages like `wip`. Keep messages concise, but prefer a brief description when adding new behavior (e.g., `add tick logging`).
- PRs should describe the hardware target (e.g., RP2350), how it was verified (commands or device output), and any flashing steps or required tools.

## Hardware & Configuration Notes
- The default runner expects a Raspberry Pi Debug Probe and a connected RP2350 device.
- `DEFMT_LOG` is set to `info` in `.cargo/config.toml`; adjust if you need more or less logging.
