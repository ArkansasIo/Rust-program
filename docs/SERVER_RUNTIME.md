# Server Runtime

## Purpose

`GameRuntime` is the authoritative simulation coordinator. Gameplay modules remain independently testable while the runtime owns the simulation clock and cross-system orchestration.

## Tick model

The default runtime is configured for 20 ticks per second. `step_n` limits work per frame through `max_ticks_per_frame` so a delayed process cannot execute an unbounded amount of simulation work in one frame.

```text
Wall clock
   ↓
Runtime scheduler
   ↓
GameRuntime::step()
   ├── advance authoritative tick
   ├── tick instances
   ├── process gameplay systems
   ├── emit/drain events
   └── update metrics
```

## Ownership

The runtime owns the authoritative tick, shard manager, instance manager, event bus, runtime metrics, and tick budget configuration.

Individual modules own their domain state. Combat owns combat resolution, quests own quest progression, and the galaxy system owns strategic topology.

## Determinism

Simulation code should avoid wall-clock-dependent gameplay calculations. Use `GameTick` or explicit simulation timestamps for state transitions, cooldowns, respawns, effects, and encounter timers.

## Scaling path

The current runtime is an in-process coordinator. A production deployment can partition the same model into gateway, authentication/session, world/shard, instance, strategic galaxy, persistence, and cache/message-bus processes.

The module boundaries are intentionally independent so those processes can be introduced without rewriting the underlying domain model.

## Testing

Runtime behavior is covered by `tests/runtime.rs`. Additional integration tests should cover tick ordering, instance lifecycle, shard allocation, session expiration, combat event delivery, persistence snapshots, reconnect/replay behavior, and overload/tick-budget handling.
