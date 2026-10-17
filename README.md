# Rust Environment API

A reusable Rust library and command-line tool for inspecting the runtime process environment.

## Features

- Read and test environment variables.
- Enumerate environment variables with heuristic redaction of likely secret values by default.
- Explicitly opt in to raw environment output for trusted local diagnostics.
- Read process arguments without assuming Unicode.
- Inspect current directory, executable path, and temporary directory.
- Report operating system, platform family, architecture, process ID, and package version.
- Export environment snapshots as JSON.
- Reusable account, menu, settings, and API-application data models.
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
```

The `vars` and `json` commands redact values whose variable names look like they may contain secrets. `--raw` disables redaction and prints a warning to stderr. The `get` command explicitly prints the requested value, so use care with sensitive variable names.

Redaction is a heuristic based on names; it is not a security boundary and may miss secrets under unfamiliar variable names. Review dumps before sharing them.

## Library

```rust
use rust_environment_api::Environment;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OS: {}", Environment::system().os);
    println!("PATH: {:?}", Environment::get("PATH"));
    println!("Executable: {}", Environment::executable()?.display());
    println!("Working directory: {}", Environment::current_dir()?.display());

    let snapshot = Environment::snapshot()?;
    println!("{}", snapshot.to_json_pretty()?);
    Ok(())
}
```

Use `Environment::variables_unredacted()` or `Environment::snapshot_unredacted()` only for trusted local diagnostics. These results can expose credentials, access tokens, passwords, and other sensitive values.

## Safety note

The API intentionally does not expose environment mutation. Environment mutation can have process-wide concurrency implications on some platforms. For child processes, prefer configuring variables through `std::process::Command`.

## Application models

The crate also exposes reusable `Account`, `MenuItem`, `Settings`, `ApiApplication`, and `ApiApplicationConfig` models. See [docs/APPLICATION.md](docs/APPLICATION.md).

## Repository

https://github.com/ArkansasIo/Rust-program
