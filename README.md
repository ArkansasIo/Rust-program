# Rust Environment API

A reusable Rust library and command-line tool for inspecting the runtime process environment.

## Features

- Read and test environment variables.
- Enumerate environment variables.
- Read process arguments without assuming Unicode.
- Inspect current directory, executable path, and temporary directory.
- Report operating system, platform family, architecture, process ID, and package version.
- Export a complete environment snapshot as JSON.
- Cross-platform Cargo project with GitHub Actions CI.

## Build

```bash
cargo build
cargo test
cargo run -- info
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
```

## Library

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

## Safety note

The API intentionally does not expose environment mutation. Environment mutation can have process-wide concurrency implications on some platforms. For child processes, prefer configuring variables through `std::process::Command`.

## Repository

https://github.com/ArkansasIo/Rust-program
