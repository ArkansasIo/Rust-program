# Development guide

## Requirements
- Rust toolchain compatible with the version in `Cargo.toml`
- Cargo (included with Rust)

## Local checks

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Project map
- `src/lib.rs`: public library exports.
- `src/main.rs`: CLI entry point.
- `src/environment/`: process environment inspection and snapshots.
- `src/account.rs`: account data model.
- `src/api.rs`: API application/configuration models.
- `src/menu.rs`: nested menu model.
- `src/settings.rs`: application settings model.
- `src/math.rs`: integer math helpers.
- `tests/`: integration tests.
- `.github/workflows/rust.yml`: cross-platform CI.

## Safe environment handling

Environment variables can contain credentials. Do not publish raw snapshots in
logs or bug reports. Prefer allow-listed variables and redact secrets in
diagnostic output. The library intentionally does not expose an environment
mutation API; configure variables on child processes when needed.
