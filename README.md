# Rust RPG / MMORPG / Sci-Fi 4X RTS Framework

A modular, server-authoritative Rust game framework for building a large-scale RPG/MMORPG combined with a persistent science-fiction 4X, RTS, and turn-based tactical universe.

## Scope

The framework now covers four interacting layers:

1. **RPG/MMORPG** — characters, attributes, sub-attributes, resources, combat, abilities, equipment, inventory, quests, progression, buffs, debuffs, threat, AI, PvE, PvP, parties, guilds, social systems, economy, crafting, trading, auction systems, instances, persistence, replication, authentication, and networking.
2. **PvE Encounter Layer** — common mobs, veterans, elites, named enemies, bosses, dungeon encounters, raid encounters, world bosses, phase mechanics, enrage timers, adds, immunities, loot, and encounter power budgets.
3. **Sci-Fi 4X / RTS Layer** — galaxies, sectors, star systems, planets, colonies, empires, factions, fleets, modular ships, research, diplomacy, trade, strategic resources, territory, AI empires, and victory conditions.
4. **Turn-Based Tactical Layer** — initiative, tactical actions, ground units, boarding actions, colony assaults, PvP scenarios, and deterministic server-side combat resolution.

## RPG Attribute Model

### Core attributes

| Attribute | Primary role |
|---|---|
| STR | Physical power, melee damage, carry capacity |
| DEX | Accuracy, attack speed, evasion, critical chance |
| CON | Health, stamina, resistance |
| INT | Spell power and magic scaling |
| WIS | Healing, mana efficiency, status resistance |
| VIT | Maximum health and survivability |
| SPI | Resource regeneration and aura strength |
| LCK | Drop/proc/variance modifiers |
| WIL | Crowd-control resistance and resolve |
| CHA | Threat, leadership and social modifiers |

### Derived/sub-attributes

Physical power, armor penetration, attack speed, accuracy, evasion, weapon handling, critical damage, spell power, elemental mastery, cast speed, magic penetration, mana efficiency, overcharge, HP regeneration, shield strength, damage reduction, block, and parry.

## Combat

Supported concepts include:

- Physical, fire, ice, lightning, earth, wind, water, light, dark, arcane, poison, bleed, void, chaos, and true damage.
- Flat resistance, percentage resistance, absorption, vulnerability, and immunity.
- Critical hits, armor mitigation, penetration, shields, damage reduction, healing, cooldowns, resources, and threat.
- Buffs, debuffs, crowd control, damage-over-time, healing-over-time, shields, immunities, and vulnerability effects.
- PvE/PvP normalization and context-specific balancing.
- Crowd-control diminishing returns: 100%, 50%, 25%, then temporary immunity.

## Equipment

Equipment slots include head, chest, legs, gloves, boots, main hand, off hand, two rings, amulet, cloak, belt, and two trinkets.

Rarities: Common, Uncommon, Rare, Epic, Legendary, Mythic.

Equipment supports separate PvE/PvP context data, item power, procs, set IDs, level requirements, cosmetics, resilience, tenacity, haste, healing, armor, magic resistance, and damage.

## Enemy and Boss Framework

Enemy ranks:

- Common
- Veteran
- Elite
- Rare
- Named
- Boss

AI archetypes:

- Brute
- Assassin
- Caster
- Controller
- Summoner
- Support
- Tank
- Sniper

Boss types:

- Dungeon
- World
- Raid
- Event
- Story
- Mythic
- Secret

Bosses support HP/time phase triggers, new abilities, adds, arena modifiers, damage-type shifts, immunities, and enrage timers.

## Formula Library

The `formulas` module contains reusable implementations for:

- Max HP
- Physical damage
- Critical chance
- Soft caps
- Diminishing returns
- Armor mitigation
- Final damage
- Hit chance
- Boss HP scaling
- Encounter power budgets
- Fleet power
- Research time
- Trade income
- Total player/empire power

Example:

```text
MaxHP = VIT × 15 + CON × 10 + GearHP
PhysicalDamage = STR × 2.5 × WeaponMultiplier
CritChance = DEX × 0.04 + GearCrit
DamageTaken = IncomingDamage × (1 - DamageMitigation)
```

## Sci-Fi 4X / RTS

### Galaxy

```text
Galaxy
└── Sectors
    └── Star Systems
        ├── Planets
        ├── Stations
        ├── Asteroid Fields
        └── Anomalies
```

### Empire attributes

Industry, Science, Economy, Influence, Logistics, Intelligence, and Stability.

### Strategic resources

Credits, Minerals, Gas, Energy, Data, Dark Matter, Antimatter, Nanites, Quantum Cores, and Alien Artifacts.

### Ships

Fighter, Corvette, Frigate, Destroyer, Cruiser, Battleship, Carrier, Dreadnought, Titan, and Flagship.

Ships are component-driven and support weapons, armor, shields, point defense, reactors, sensors, engines, AI cores, and targeting computers.

### Research

Physics, Engineering, Biotech, Psionics, Economics, Military, and Computing research branches are supported.

### Diplomacy

Neutral, Trade, Non-Aggression, Research Pact, Alliance, Vassal, and War relations are modeled.

### Empire AI

Expansionist, Defensive, Economic, Aggressive, Technological, and Diplomatic strategy styles are supported.

## Turn-Based Tactical Combat

Units calculate initiative from speed, tactical skill, and equipment bonus. Actions include Move, Attack, Ability, Overwatch, Use Item, Defend, and Interact.

The system can be used for boarding actions, colony invasions, base assaults, tactical missions, and arena/PvP battles.

## Server Architecture

```text
Client
  ↓
Network / Authentication / Rate Limit
  ↓
Authoritative Game Server
  ├── RPG Character Service
  ├── Combat Service
  ├── AI Service
  ├── Encounter Service
  ├── World / Shard Service
  ├── Galaxy / Empire Service
  ├── Fleet / Space Combat Service
  ├── Tactical Turn Service
  ├── Social / Guild / Chat Service
  ├── Economy / Trade Service
  └── Persistence / Replication
  ↓
Database + Cache + Telemetry
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
└── existing RPG/MMO modules...
```

## Build

```bash
cargo check
cargo test
cargo run
```

## Design Principles

- Server authoritative simulation.
- Deterministic combat calculations where practical.
- Data-driven content.
- Modular systems with explicit ownership boundaries.
- PvE and PvP rules can diverge without duplicating the entire combat engine.
- Persistent MMO state is separated from transient encounter state.
- Strategic galaxy simulation is independent of real-time tactical instances.
- Formulas are centralized for balance and telemetry.

## Status

This repository is an expanding framework/scaffold rather than a finished commercial MMO server. The modules define the core data model and deterministic logic needed to continue toward production services, database adapters, network transport, dedicated server binaries, live configuration, and load-tested infrastructure.

## License

MIT
