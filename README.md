# Rust RPG / MMORPG / Sci-Fi 4X RTS Framework

A modular, server-authoritative Rust game framework for building a large-scale RPG/MMORPG combined with a persistent science-fiction 4X, RTS, and turn-based tactical universe.

## Scope

The framework now covers four interacting layers:

1. **RPG/MMORPG** — characters, attributes, resources, combat, abilities, equipment, inventory, quests, progression, effects, threat, AI, PvE, PvP, parties, guilds, social systems, economy, crafting, trading, auctions, instances, persistence, replication, authentication, networking, and shards.
2. **PvE Encounter Layer** — common mobs, veterans, elites, named enemies, bosses, dungeon encounters, raid encounters, world bosses, phase mechanics, enrage timers, adds, immunities, loot, and encounter power budgets.
3. **Sci-Fi 4X / RTS Layer** — galaxies, sectors, star systems, planets, colonies, empires, fleets, modular ships, research, diplomacy, trade, strategic resources, AI empires, events, and victory conditions.
4. **Turn-Based Tactical Layer** — initiative, tactical actions, ground units, boarding actions, colony assaults, PvP scenarios, and deterministic server-side combat resolution.

## RPG Attribute Model

Core attributes include STR, DEX, CON, INT, WIS, VIT, SPI, LCK, WIL, and CHA. Derived values cover physical power, penetration, attack speed, accuracy, evasion, critical scaling, spell power, elemental mastery, regeneration, shields, mitigation, block, parry, threat, and leadership.

## Combat

Supported concepts include physical and elemental damage, resistance and vulnerability, critical hits, armor mitigation, penetration, shields, healing, cooldowns, resources, threat, buffs, debuffs, crowd control, damage-over-time, healing-over-time, immunities, and PvE/PvP normalization.

Crowd-control diminishing returns use the configured progression of 100%, 50%, 25%, followed by temporary immunity.

## Equipment and Enemies

Equipment supports standard RPG slots, item power, rarity, PvE/PvP context, procs, sets, requirements, resilience, tenacity, haste, healing, armor, magic resistance, and damage.

Enemy ranks include Common, Veteran, Elite, Rare, Named, and Boss. AI archetypes include Brute, Assassin, Caster, Controller, Summoner, Support, Tank, and Sniper. Boss definitions support phases, adds, immunities, arena modifiers, damage-type shifts, and enrage timers.

## Formula Library

The `formulas` module centralizes reusable calculations for max HP, physical damage, critical chance, soft caps, diminishing returns, armor mitigation, final damage, hit chance, boss scaling, encounter budgets, fleet power, research time, trade income, and total power.

Example:

```text
MaxHP = VIT × 15 + CON × 10 + GearHP
PhysicalDamage = STR × 2.5 × WeaponMultiplier
CritChance = DEX × 0.04 + GearCrit
DamageTaken = IncomingDamage × (1 - DamageMitigation)
```

## Sci-Fi 4X / RTS

The strategic layer models:

- Galaxy → sectors → star systems → planets/stations/anomalies.
- Empire attributes: Industry, Science, Economy, Influence, Logistics, Intelligence, Stability.
- Strategic resources and planet buildings.
- Fighter through Flagship ship classes with component-driven construction.
- Fleets, sensors, morale, command capacity, and combat power.
- Physics, Engineering, Biotech, Psionics, Economics, Military, and Computing research.
- Neutral, Trade, Non-Aggression, Research Pact, Alliance, Vassal, and War relations.
- Expansionist, Defensive, Economic, Aggressive, Technological, and Diplomatic AI styles.
- Persistent galaxy events and victory conditions.

## Runtime

`GameRuntime` provides a deterministic server-side tick loop with configurable tick rate, per-frame work limits, instance ticking, shard management, event bus ownership, and runtime metrics.

```rust
let mut runtime = rust_battle_engine::GameRuntime::default();
runtime.step();
runtime.step_n(4);
```

## Server Architecture

```text
Client
  ↓
Network / Authentication / Rate Limit
  ↓
Authoritative Game Server
  ├── RPG Character Service
  ├── Combat / Ability / Effects
  ├── AI / Threat
  ├── Encounter / Dungeon / Raid
  ├── World / Shard / Instance
  ├── Galaxy / Empire / Planet
  ├── Fleet / Space Combat
  ├── Research / Diplomacy / Strategy
  ├── Tactical Turn Service
  ├── Social / Guild / Chat
  ├── Economy / Trade / Auction
  └── Persistence / Replication
  ↓
MySQL + Redis + Telemetry
```

## Repository Structure

```text
src/
├── attributes.rs       # Core and derived attributes
├── formulas.rs         # RPG/MMO/4X formulas
├── resources.rs        # HP, MP, stamina and class resources
├── resistances.rs      # Elemental resistance model
├── effects.rs          # Buff/debuff/CC effects
├── equipment.rs        # Item power and equipment loadouts
├── enemies.rs          # Enemy ranks and AI archetypes
├── boss.rs             # Boss phases and enrage
├── encounters.rs       # Dungeon/raid encounter definitions
├── galaxy.rs           # Galaxy/sector/system model
├── empire.rs           # 4X empire model
├── planet.rs           # Colonies and buildings
├── ships.rs            # Modular ships/components
├── fleet.rs            # Fleet command and power
├── space_combat.rs     # Space combat resolution
├── research.rs         # Technology trees
├── diplomacy.rs        # Diplomacy agreements
├── strategy.rs         # Empire AI and victory conditions
├── turn_based.rs       # Tactical turn engine
├── galaxy_events.rs    # Persistent universe events
├── runtime.rs          # Authoritative simulation tick loop
└── existing RPG/MMO modules...

database/schema.sql     # MySQL persistence schema
data/                   # Seed/content tables
config/                 # Runtime and balance configuration
docs/                   # Architecture and game-design documentation
tests/                  # Integration tests
docker-compose.yml      # Local MySQL + Redis development stack
```

## Local Infrastructure

```bash
docker compose up -d
cargo check
cargo test
cargo run
```

The compose stack provisions MySQL 8.4 and Redis 7 for local development. The Rust framework currently keeps database/network dependencies optional at the architecture level so the core simulation remains lightweight and deterministic.

## CI

GitHub Actions validates formatting, compilation, Clippy warnings, and tests on Ubuntu, Windows, and macOS. The workflow formats the workspace before compilation so generated modules do not block the validation pipeline on formatting alone.

## Design Principles

- Server-authoritative simulation.
- Deterministic combat calculations where practical.
- Data-driven content.
- Modular systems with explicit ownership boundaries.
- PvE and PvP rules can diverge without duplicating the entire combat engine.
- Persistent MMO state is separated from transient encounter state.
- Strategic galaxy simulation is independent of tactical instances.
- Formulas are centralized for balance and telemetry.
- Runtime orchestration is separated from individual gameplay systems.

## Status

This repository is an expanding framework/scaffold rather than a finished commercial MMO server. The current codebase provides the core data model, combat/RPG systems, MMO services, strategic systems, tactical systems, runtime loop, database schema, local infrastructure, documentation, and CI foundation needed for continued production development.

## License

MIT
