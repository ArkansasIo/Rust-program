# Build Fixes — 2026-10-24

## CI failure diagnosis

The latest GitHub Actions run failed at `cargo check --all-targets --all-features`.
The compiler reported these blockers:

- `Battle` did not implement `Debug`, while `GameEngine` derived it.
- `FriendService::remove` left boolean-valued `HashSet::remove` expressions as the final expressions in unit-returning branches.
- `MatchMode` was used as a `HashMap` key without deriving `Hash`.
- `SessionManager::connect` and `QuestLog::accept` left `HashMap::insert` return values in unit-returning functions.
- `World` derived `Default`, but `GameTick` did not implement it.

## Changes made

The accompanying patch fixes those type/trait errors. It also makes the battle tick explicitly simultaneous: every attack is calculated from the start-of-tick snapshot before any damage is applied. This prevents an earlier vector index from suppressing a same-tick counterattack.

Quest completion was tightened so exploration and delivery objectives cannot silently count as complete without a progress model. As this patch does not yet add quest-progress event APIs, those objective kinds remain incomplete until such an API is implemented.

## Validation

GitHub Actions must run `cargo fmt --all`, `cargo check --all-targets --all-features`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-features` against the committed changes. No local Cargo build was executed as part of this repository write.
