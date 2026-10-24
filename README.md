# Rust RPG / MMORPG / Sci-Fi 4X RTS Framework

A modular, server-authoritative Rust framework for a large-scale RPG/MMORPG combined with persistent science-fiction 4X, RTS, and turn-based tactical systems.

## Core Scope

The framework is organized into interoperating layers:

1. **RPG/MMORPG** — characters, classes, attributes, sub-attributes, resources, combat, abilities, equipment, inventory, quests, progression, effects, threat, AI, PvE, PvP, parties, guilds, social systems, economy, crafting, trading, auctions, instances, persistence, replication, authentication, networking, and shards.
2. **PvE Encounter Layer** — common mobs, veterans, strong mobs, elites, named enemies, bosses, dungeon encounters, raid encounters, world bosses, phase mechanics, enrage timers, adds, immunities, loot, and encounter power budgets.
3. **Sci-Fi 4X / RTS Layer** — galaxies, sectors, star systems, planets, colonies, empires, fleets, modular ships, research, diplomacy, trade, strategic resources, AI empires, events, and victory conditions.
4. **Turn-Based Tactical Layer** — initiative, tactical actions, ground units, boarding actions, colony assaults, PvP scenarios, and deterministic server-side combat resolution.

## RPG Stat Architecture

The combat model follows:

```text
Core Attributes
    ↓
Sub-Attributes
    ↓
Combat Stats
    ↓
Resources + Resistances
    ↓
Equipment / Talents / Skills
    ↓
Buffs + Debuffs
    ↓
PvE/PvP Context
    ↓
Authoritative Combat Result
```

### Core Attributes

- **STR** — physical power and melee scaling
- **DEX** — accuracy, attack speed, evasion, critical scaling
- **CON** — health, stamina, physical resistance
- **INT** — spell power and magical scaling
- **WIS** — healing, mana sustain, status resistance
- **VIT** — maximum HP and survivability
- **SPI** — resource regeneration and aura strength
- **LCK** — proc, loot, and luck effects
- **WIL** — crowd-control resistance and resolve
- **CHA** — threat, NPC interaction, leadership

### Sub-Attributes

Physical: Physical Power, Armor Penetration, Attack Speed, Accuracy, Evasion, Weapon Handling, Critical Chance, Critical Damage.

Magical: Spell Power, Elemental Mastery, Cast Speed, Magic Penetration, Mana Efficiency, Overcharge, Healing Power, Status Power.

Defensive: Max HP, HP Regen, Shield Strength, Damage Reduction, Block Chance, Block Value, Parry Chance, Tenacity, Poise.

Utility: Movement Speed, Cooldown Reduction, Resource Regeneration, Threat Generation, Threat Reduction, Detection, Stealth, Command Capacity.

## Combat and Resources

Supported damage types include Physical, Fire, Ice, Lightning, Earth, Wind, Water, Light, Dark, Arcane, Poison, Bleed, Void, Chaos, and True Damage.

Resources include HP, MP, Stamina, Energy, Rage, Focus, Faith, Aether, Heat, and Sanity.

The framework supports flat resistance, percentage resistance, penetration, vulnerability, absorption, shields, immunities, critical hits, armor mitigation, healing, cooldowns, threat, buffs, debuffs, crowd control, damage-over-time, healing-over-time, and PvE/PvP normalization.

Default crowd-control diminishing returns:

```text
1st CC = 100%
2nd CC = 50%
3rd CC = 25%
4th CC = temporary immunity
```

## Buffs and Debuffs

Buff examples: Power Surge, Berserk, Arcane Amplify, Precision, Shielded, Fortified, Regeneration, Damage Absorption, Invulnerability, Haste, Stealth, Invisibility, True Sight, Phase Shift.

Debuff examples: Stun, Freeze, Root, Silence, Fear, Charm, Sleep, Knockback, Knockdown, Burn, Poison, Bleed, Corruption, Frostbite, Shock, Weakness, Frailty, Hex, Slow, Cripple, Curse.

Seed definitions are in `data/status_effects.csv`.

## Equipment

Slots include Head, Chest, Legs, Gloves, Boots, Main Hand, Off Hand, Ring 1, Ring 2, Amulet, Cloak, Belt, Trinket 1, and Trinket 2.

Rarity progression:

```text
Common → Uncommon → Rare → Epic → Legendary → Mythic
```

Equipment supports item power, stat rolls, requirements, sockets, set bonuses, procs, cosmetic metadata, PvE/PvP statistics, and scaling rules.

## Combat Formulas

```text
MaxHP = VIT × 15 + CON × 10 + GearHP
PhysicalDamage = STR × 2.5 × WeaponMultiplier
CritChance = DEX × 0.04 + GearCrit
DamageTaken = IncomingDamage × (1 - DamageMitigation)
```

Stat power:

```text
StatValuePU = RawStat × Weight × ScalingCurve
```

Soft cap:

```text
EffectiveStat = Raw / (Raw + SoftCap)
```

Armor mitigation:

```text
Mitigation = Armor / (Armor + K × AttackerLevel)
```

The centralized formula library is intended to keep balancing, tests, and telemetry consistent.

## Enemy / Mob / Boss Framework

### Enemy ranks

| Rank | Baseline | Purpose |
|---|---:|---|
| Common | 60–80% | Farming, quests, XP |
| Veteran | 120% | Stronger world/dungeon enemy |
| Strong | 150% | Event/dungeon enemy |
| Elite | 200–300% | Mini-boss / coordination |
| Named | ~250%+ | Unique encounter and loot |
| Boss | 500–5000% | Dungeon, raid, world and story encounters |

AI archetypes include Brute, Assassin, Caster, Controller, Summoner, Support, Tank, and Sniper.

Boss types include Dungeon Boss, World Boss, Raid Boss, Event Boss, Story Boss, Mythic Boss, and Secret Boss.

Boss phases can trigger from HP thresholds, time, player deaths, objectives, or environment changes. Phases can add abilities, change arenas, spawn adds, shift damage types, modify resistances, and activate enrage mechanics.

Seed tables:

- `data/enemy_archetypes.csv`
- `data/boss_templates.csv`

## Encounter Lifecycle

```text
Spawn
 → Detect Players
 → Acquire Threat
 → Combat
 → Phase Evaluation
 → Ability Selection
 → Effects / Damage
 → Adds / Objectives
 → Death / Wipe
 → Rewards
 → Cleanup
 → Reset / Respawn
```

Encounter power budget:

```text
EncounterPowerBudget = Σ(PlayerPower) × DifficultyFactor
```

Boss health scaling:

```text
BossHP = Base × (1 + LevelDiff × 0.18) × PartyMult × DifficultyMult
PartyMult = 1 + (Players - 1) × 0.65
```

## Sci-Fi 4X / RTS

The strategic layer models:

- Galaxy → sectors → star systems → planets/stations/anomalies.
- Empire attributes: Industry, Science, Economy, Influence, Logistics, Intelligence, Stability.
- Credits, minerals, gas, energy, data, dark matter, antimatter, nanites, quantum cores, and artifacts.
- Planet buildings and population.
- Fighter through Flagship ship classes.
- Modular weapons, armor, shields, reactors, sensors, engines, AI cores, and targeting computers.
- Fleets, sensors, morale, command capacity, and combat power.
- Research and technology trees.
- Diplomacy, alliances, trade agreements, non-aggression pacts, research pacts, vassalization, and war.
- Strategic AI personalities and galaxy events.
- Military, economic, technological, diplomatic, and ascension victory paths.

## Turn-Based Tactical Layer

Ground and boarding combat supports:

- Infantry
- Mechs
- Drones
- Vehicles
- Heroes

Example initiative:

```text
Initiative = Speed + TacticalSkill + EquipmentBonus
```

Actions include Move, Attack, Ability, Overwatch, and Use Item.

## MMO Server Architecture

```text
Client
  ↓
Network / Authentication / Rate Limit
  ↓
Authoritative Game Server
  ├── Character / Stats
  ├── Combat / Abilities / Effects
  ├── AI / Threat
  ├── Dungeon / Raid / Encounter
  ├── World / Shard / Instance
  ├── Galaxy / Empire / Planet
  ├── Fleet / Space Combat
  ├── Research / Diplomacy / Strategy
  ├── Tactical Turn Service
  ├── Social / Guild / Chat
  ├── Economy / Trade / Auction
  └── Persistence / Replication / Telemetry
  ↓
MySQL + Redis + Metrics
```

## Repository Structure

```text
src/
├── attributes.rs       # Core and derived attributes
├── formulas.rs         # RPG/MMO/4X formulas
├── resources.rs        # Resource pools
├── resistances.rs      # Resistance model
├── effects.rs          # Buff/debuff/effect resolution
├── equipment.rs        # Equipment and item power
├── enemies.rs          # Enemy ranks and archetypes
├── boss.rs             # Boss phases and enrage
├── encounters.rs       # Encounter definitions
├── loot.rs             # Loot generation
├── threat.rs           # Aggro/threat
├── ai.rs               # AI controllers
├── runtime.rs          # Authoritative simulation tick loop
├── server.rs           # Server facade
├── matchmaking.rs      # PvP/PvE queues
├── replication.rs      # State replication
├── rate_limit.rs       # Request limiting
├── telemetry.rs        # Runtime counters
├── galaxy.rs           # Galaxy / sectors / systems
├── empire.rs           # Empire management
├── planet.rs           # Planet / colony systems
├── ships.rs            # Ship classes/components
├── fleet.rs            # Fleet management
├── space_combat.rs     # Space combat
├── research.rs         # Technology trees
├── diplomacy.rs        # Diplomacy
├── strategy.rs         # Strategic AI and victory
├── turn_based.rs       # Tactical turn engine
└── galaxy_events.rs    # Persistent universe events

data/
├── rpg_stats.csv
├── status_effects.csv
├── enemy_archetypes.csv
├── boss_templates.csv
└── existing content tables

docs/
├── RPG_MMO_COMBAT_FRAMEWORK.md
├── MASTER_GAME_DESIGN.md
├── COMBAT_AND_BALANCE.md
├── SCI_FI_4X_RTS.md
└── SERVER_RUNTIME.md
```

## Runtime

```rust
let mut runtime = rust_battle_engine::GameRuntime::default();
runtime.step();
runtime.step_n(4);
```

The runtime provides a deterministic authoritative tick loop, instance ticking, shard management, event ownership, and runtime metrics.

## Local Development

```bash
docker compose up -d
cargo fmt --all -- --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run
```

## Design Principles

- Server-authoritative simulation.
- Deterministic combat calculations where practical.
- Data-driven content.
- Explicit module ownership boundaries.
- Separate PvE and PvP context rules without duplicating the whole combat engine.
- Persistent MMO state separated from transient encounter state.
- Strategic galaxy simulation separated from tactical instances.
- Centralized formulas for balance and telemetry.
- Runtime orchestration separated from gameplay systems.
- Configurable caps, curves, scaling, and seasonal modifiers.

## Documentation

The expanded combat specification is in [`docs/RPG_MMO_COMBAT_FRAMEWORK.md`](docs/RPG_MMO_COMBAT_FRAMEWORK.md). It contains the full attribute, sub-stat, resource, resistance, buff/debuff, equipment, enemy, boss, encounter, AI, scaling, and server-authority model supplied for this project.

## Status

This repository is an expanding production-oriented framework/scaffold rather than a finished commercial MMO server. The codebase provides the core RPG/MMO systems, PvE/PvP model, strategic 4X/RTS systems, tactical systems, runtime/server foundation, content tables, documentation, and infrastructure required for continued implementation.

## License

MIT
