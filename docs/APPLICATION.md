# Application Account, Menus, Settings, and API

The Rust Environment API now includes a reusable application model suitable for a CLI, web UI, desktop application, or future server API.

## Account

An account contains:

- stable account ID
- username
- display name
- email address
- roles
- enabled/disabled state

Roles currently support ordinary users and administrators. Authentication and password storage are intentionally not implemented in this library; production authentication should use a dedicated identity provider or secure password-hashing service.

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
- Authentication model: bearer token
- Rate limit: 60 requests/minute
- Scopes: `read`, `write`, `math`, `environment`

No real bearer tokens or credentials are committed to this repository. Token issuance, rotation, hashing, revocation, and persistence belong in the server/authentication layer.

## CLI

```text
rust-env account
rust-env menu
rust-env settings
rust-env api
rust-env api-app
```

Each command emits JSON suitable for another application to consume.
