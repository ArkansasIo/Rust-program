# Rust Environment + Mathematics API

A reusable Rust library and command-line tool for inspecting process environment data and providing checked integer mathematics helpers.

## Features

- Read and enumerate environment variables and inspect process arguments.
- Inspect the current directory, executable path, temporary directory, OS, architecture, and process ID.
- Export an environment snapshot as JSON.
- Account, API application, settings, and nested-menu data models.
- Integer mathematics helpers: prime testing, even/odd classification, greatest common divisor, and checked arithmetic.
- Cross-platform GitHub Actions CI.

## Build and test

```bash
cargo build
cargo test
cargo run -- info
```

Run formatting and lint checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## CLI

```text
rust-env info
rust-env vars
rust-env get PATH
rust-env args
rust-env cwd
rust-env executable
rust-env temp
rust-env system
rust-env json
rust-env account
rust-env menu
rust-env settings
rust-env api
rust-env api-app
rust-env math prime 97
rust-env math classify 42
rust-env math gcd 54 24
rust-env math calc mul 6 7
```

The math calculator uses checked `i64` arithmetic. Division requires an integral result; division by zero, overflow, and unknown operators return an error instead of silently truncating.

## Library example

```rust
use rust_environment_api::Environment;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OS: {}", Environment::system().os);
    println!("PATH: {:?}", Environment::get("PATH"));
    println!("Executable: {}", Environment::executable()?.display());
    println!("Working directory: {}", Environment::current_dir()?.display());
    Ok(())
}
```

Math API details: [`docs/MATHEMATICS.md`](docs/MATHEMATICS.md).
Development guide: [`docs/DEVELOPMENT.md`](docs/DEVELOPMENT.md).

## Safety note

Environment variables may contain credentials. Treat environment snapshots as sensitive and redact secrets before sharing. This library intentionally avoids a public process-environment mutation API. Configure child-process variables using `std::process::Command`.

## Repository

https://github.com/ArkansasIo/Rust-program
