# Rust Battle Engine — RPG/MMORPG Framework

A modular, server-authoritative Rust gameplay framework for building RPG, action-RPG, MMO, and MMORPG projects.

## What is included

### Core simulation
- `core` — entity IDs, teams, ticks, vectors
- `entities` — combatant runtime state
- `stats` — attributes, combat stats, resources
- `combat` — attacks, damage, mitigation, critical hits and death
- `abilities` — abilities, range, costs and cooldowns
- `ai` — target selection and combat decisions
- `battle` — deterministic battle loop
- `physics` — movement/range primitives
- `world` — world entity storage and ticks
- `events` — event bus
- `status` — buffs, debuffs, crowd control, damage-over-time and regeneration
- `threat` — threat/aggro tables

### RPG systems
- `character` — persistent character model
- `rpg` — classes, skills, talents, quests and progression
- `progression` — XP, levels and skill points
- `items` / `inventory` — items, equipment and inventory
- `loot` — deterministic weighted loot tables
- `economy` — wallets, recipes, crafting and auction house
- `quests` — quest objectives and state
- `achievements` — achievement tracking
- `trade` — player-to-player trade offers
- `pvp` — duel, arena, battleground and ranked match models

### MMORPG systems
- `auth` — accounts and server sessions
- `mmorpg` — player sessions, saves, mail and matchmaking
- `shard` — realm/shard capacity and allocation
- `instance` — dungeon/raid instance lifecycle
- `replication` — per-player entity state deltas
- `network` — transport-neutral client/server protocol
- `chat` — channels and moderation hooks
- `friends` — friend requests and social graph
- `social` — parties and guilds
- `npc` — NPC registry and archetypes
- `dialogue` — dialogue trees and choices
- `world_content` — zones, spawns, dungeons and raids
- `security` — sequence/rate-limit and anti-cheat hooks
- `persistence` — repository abstraction and JSON serialization

## Architecture

```text
Client
  │
  ▼
Protocol / Sequence Guard / Rate Limit
  │
  ▼
Auth + Session
  │
  ▼
Command Validation
  │
  ▼
Authoritative World Simulation
  ├── Combat / AI / Status / Threat
  ├── Character / Inventory / Quest / Economy
  ├── NPC / Dialogue / Dungeon / Raid
  └── PvP / Party / Guild / Trade
  │
  ├── Event Bus
  ├── Persistence Snapshot
  └── Replication Delta
  │
  ▼
Clients
```

## Database

A starter relational schema is included at `database/schema.sql`. It covers accounts, sessions, characters, XP, wallets, inventory, quests, skills, guilds, friends, mail and audit events.

The Rust code intentionally uses a storage-neutral `Repository<K,V>` abstraction. A production deployment can add MySQL/PostgreSQL and Redis adapters without coupling gameplay code to a database driver.

## Documentation

- `docs/ARCHITECTURE.md` — complete runtime architecture
- `docs/RPG_SYSTEMS.md` — RPG systems specification
- `docs/MMORPG_BACKEND.md` — MMORPG service/backend specification
- `docs/DATA_MODEL.md` — persistent and runtime data model
- `docs/API.md` — public API surface
- `docs/ROADMAP.md` — implementation roadmap
- `database/schema.sql` — relational database starting schema

## Build and test

```bash
cargo fmt --all -- --check
cargo check
cargo test
cargo run
```

The sample executable runs a small deterministic battle using the public engine APIs.

## Design principles

1. Server-authoritative gameplay.
2. Deterministic simulation where practical.
3. Data-driven content.
4. Modular systems with explicit boundaries.
5. Events for cross-system communication.
6. Transport and storage abstractions around the gameplay core.
7. Testable simulation without requiring a running database or network.

## License

MIT
