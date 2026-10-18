# Rust Environment API

A reusable Rust library and command-line tool for inspecting the runtime process environment.

## Features

- Read and test environment variables.
- Enumerate environment variables, with likely credential values redacted by default.
- Explicitly opt in to raw environment output for trusted local diagnostics.
- Read process arguments without assuming Unicode.
- Inspect the current directory, executable path, and temporary directory.
- Report operating system, platform family, architecture, process ID, and package version.
- Export environment snapshots as JSON.
- Provide reusable account, menu, settings, and API-application data models.
- Cross-platform Cargo project with GitHub Actions CI.

## Build and test

Requires Rust 1.85 or newer.

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo run -- info
```

## CLI

```text
rust-env info
rust-env vars
rust-env vars --raw
rust-env get PATH
rust-env args
rust-env cwd
rust-env executable
rust-env temp
rust-env system
rust-env json
rust-env json --raw
rust-env account
rust-env menu
rust-env settings
rust-env api
rust-env api-app
rust-env help
```

The `vars` and `json` commands redact values whose variable names appear likely to contain secrets. The `--raw` variants disable redaction and print a warning because their output may expose secrets. The `get` command intentionally prints the requested value; it warns when the name looks sensitive.

Redaction is heuristic rather than a security boundary. It may miss credentials stored under unfamiliar names, and other fields such as paths or process arguments can still be sensitive. Review output before sharing diagnostics.

## Library example

```rust
use rust_environment_api::Environment;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OS: {}", Environment::system().os);
    println!("PATH: {:?}", Environment::get("PATH")?);
    println!("Executable: {}", Environment::executable()?.display());
    println!("Working directory: {}", Environment::current_dir()?.display());

    let snapshot = Environment::snapshot()?;
    println!("{}", snapshot.to_json_pretty()?);
    Ok(())
}
```

Use `Environment::variables_unredacted()` or `Environment::snapshot_unredacted()` only for trusted local diagnostics; they can expose credentials, access tokens, passwords, and other sensitive values.

## Environment mutation

The API intentionally does not expose environment mutation. Environment mutation can have process-wide concurrency implications on some platforms. For child processes, prefer configuring variables through `std::process::Command`.

## Application models

The crate also exposes reusable `Account`, `MenuItem`, `Settings`, `ApiApplication`, and `ApiApplicationConfig` models. See [docs/APPLICATION.md](docs/APPLICATION.md).

## Repository

https://github.com/ArkansasIo/Rust-program
