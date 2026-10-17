# Application Account, Menus, Settings, and API

The Rust Environment API includes reusable application models suitable for a CLI, web UI, desktop application, or future server API.

## Account

An account contains a stable account ID, username, display name, email address, roles, and enabled/disabled state. Roles currently support ordinary users and administrators.

Authentication and password storage are intentionally not implemented in this library. Production authentication should use a dedicated identity provider or secure password-hashing service.

## Main menu

- Home
- Account
  - Profile
  - Security
  - Sessions
  - API Applications
- Mathematics
  - Calculator
  - Algebra
  - Calculus
  - Equations
  - Logic
  - Linear Algebra
  - Statistics
  - Units
- Environment
  - System
  - Variables
  - Process
  - Snapshot
- Settings
  - General
  - Appearance
  - Privacy
  - Notifications
  - Developer
- Help
  - Documentation
  - API Reference
  - About

Menus are represented as a recursive `MenuItem` tree, allowing unlimited submenu depth.

## Settings

The default settings model provides language, theme, notifications, telemetry, developer mode, and API enablement. These are configuration values, not secrets.

## API applications

An `ApiApplication` represents an application registered against the service. It contains an ID, name, description, owning account, scopes, and enabled state.

The default API configuration uses:

- Base path: `/api`
- Version: `v1`
- Authentication model label: bearer token
- Rate limit setting: 60 requests/minute
- Scopes: `read`, `write`, `math`, `environment`

These settings describe configuration only; this crate does not implement an HTTP server, enforce rate limits, issue bearer tokens, or authenticate requests. Token issuance, rotation, hashing, revocation, and persistence belong in a server/authentication layer.

## Environment privacy

The CLI `vars` and `json` commands redact likely secret values by default. The matching is heuristic and relies on variable names, so it cannot guarantee that every credential will be found. The `--raw` flag and the library's `variables_unredacted()` / `snapshot_unredacted()` methods intentionally expose raw values and should be used only for trusted local diagnostics.

The `get` command prints the requested value directly by design. Avoid using it for credentials in shared terminals, logs, or recordings.

## CLI

```text
rust-env account
rust-env menu
rust-env settings
rust-env api
rust-env api-app
```

Each command emits JSON suitable for another application to consume.
