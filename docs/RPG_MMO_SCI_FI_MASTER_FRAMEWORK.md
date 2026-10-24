# RPG / MMORPG / Sci-Fi 4X / RTS / Turn-Based Master Framework

## Purpose

This document defines the unified game framework for the Rust-program engine. It combines the RPG/MMORPG character layer with PvE/PvP combat, enemies and bosses, persistent MMO services, fleets, planets, empires, 4X strategy, RTS space combat, and turn-based tactical combat.

## 1. Architecture

```text
Galaxy
 └─ Sector
    └─ Star System
       ├─ Planet / Station / Anomaly
       ├─ Fleet / RTS Battle
       └─ Ground / Boarding Instance
          └─ Character / Party / Encounter
```

The authoritative Rust server owns simulation state. Clients send intents and receive validated state changes.

## 2. Character Attributes

### Primary attributes

- STR — physical power and melee scaling
- DEX — accuracy, evasion, critical chance and speed
- CON — health, stamina and resistance scaling
- INT — spell and technology power
- WIS — healing, mana sustain and status resistance
- VIT — maximum health and survivability
- SPI — resource regeneration and aura strength
- LCK — proc, loot and variance modifiers
- WIL — resolve and crowd-control resistance
- CHA — threat, social and leadership modifiers

### Secondary attributes

Physical: physical power, penetration, attack speed, accuracy, evasion, weapon handling, critical damage.

Magical: spell power, elemental mastery, cast speed, magic penetration, mana efficiency, overcharge.

Survival: max HP, HP regeneration, shields, mitigation, block, parry, tenacity, poise.

Utility: movement speed, cooldown reduction, resource regeneration, threat, perception, stealth detection, carrying capacity.

## 3. Combat Statistics

Offensive values include base damage, skill scaling, critical chance, critical damage, combo multiplier, backstab multiplier, weak-point damage and elemental bonuses.

Defensive values include armor, magic resistance, damage mitigation, guard strength, dodge, block value, parry and shield strength.

Tempo values include attack speed, cast speed, cooldown reduction, global cooldown, movement speed and initiative.

## 4. Resources

Core pools:

- HP
- MP
- Stamina

Class resources:

- Energy
- Rage
- Focus
- Faith
- Aether
- Heat
- Sanity

The resource subsystem must support maximum value, current value, regeneration, drain, reservation, overflow and resource-specific rules.

## 5. Damage and Resistance

Damage types:

Physical, Fire, Ice, Lightning, Earth, Wind, Water, Light, Dark, Arcane, Poison, Bleed, Void, Chaos and True.

Resistance layers:

1. Flat reduction
2. Percentage reduction
3. Penetration
4. Absorption / shields
5. Immunity or vulnerability
6. Final damage modifier

## 6. Buffs and Debuffs

### Buff families

- Power Surge — damage increase
- Berserk — attack speed with defensive tradeoff
- Arcane Amplify — spell power
- Precision — critical chance
- Shielded — temporary shield
- Fortified — mitigation
- Regeneration — periodic healing
- Damage Absorption — damage barrier
- Haste — tempo increase
- Stealth / Invisibility
- Flight
- True Sight
- Phase Shift

### Debuff families

Crowd control: stun, freeze, root, silence, fear, charm, sleep, knockback and knockdown.

Damage over time: burn, poison, bleed, corruption, frostbite and shock.

Attribute control: weakness, frailty, hex, slow, cripple and curse.

PvP and boss CC diminishing returns: 100%, 50%, 25%, then temporary immunity.

## 7. Enemy Taxonomy

| Tier | Baseline | Purpose |
|---|---:|---|
| Common | 60–80% | farming, quests, XP |
| Veteran | 120–150% | advanced world/dungeon combat |
| Elite | 200–300% | mini-boss/coordination encounters |
| Rare/Named | variable | special mechanics and loot |
| Boss | 500–5000% | major encounters |

Enemy AI archetypes:

- Brute
- Assassin
- Caster
- Controller
- Summoner
- Support
- Tank
- Sniper
- Scout
- Healer
- Artillery
- Swarm
- Commander

## 8. Boss Framework

Boss categories:

- Dungeon Boss
- World Boss
- Raid Boss
- Event Boss
- Story Boss
- Mythic Boss
- Secret Boss

Phase triggers may include HP thresholds, elapsed time, player deaths, objectives and environmental events.

Typical phase sequence:

```text
Phase 1 → baseline mechanics
Phase 2 → enrage + adds
Phase 3 → arena/environment change
Phase 4 → ultimate mechanic
```

Boss definitions support abilities, phases, adds, immunities, vulnerabilities, arena modifiers, damage-type changes, enrage timers, telegraphs and counterplay.

## 9. Encounter System

Encounter power is derived from participating player power and difficulty:

```text
EncounterPowerBudget = Sum(PlayerPower) × DifficultyFactor
```

Encounter directors may control spawn groups, reinforcement waves, objectives, hazards, rewards, timers and scaling.

## 10. Equipment

Slots:

- Head
- Chest
- Legs
- Gloves
- Boots
- Main Hand
- Off Hand
- Ring 1
- Ring 2
- Amulet
- Cloak
- Belt
- Trinket 1
- Trinket 2

Rarity:

Common → Uncommon → Rare → Epic → Legendary → Mythic

Items support PvE stats, PvP stats, procs, scaling rules, requirements, sockets, set bonuses, cosmetic data and durability.

## 11. Item Power

```text
StatValuePU = RawStat × Weight × ScalingCurve
WeaponPower = BaseDamage × SpeedFactor + AttributeScaling + ElementScaling
```

PvP values may be normalized independently from PvE values.

## 12. PvE / PvP Rules

PvE emphasizes progression, build diversity and encounter mechanics.

PvP emphasizes predictable counterplay, normalization, burst control, reduced RNG, resilience, tenacity and healing controls.

Hard caps must be data-driven and configurable rather than embedded throughout gameplay code.

## 13. MMORPG Systems

The framework includes:

- accounts and authentication
- character progression
- classes and subclasses
- abilities and talents
- inventory and equipment
- quests and achievements
- parties and guilds
- friends and chat
- mail
- crafting
- trading
- auction house
- PvP matchmaking
- dungeon and raid instances
- shards and world state
- persistence
- replication
- telemetry
- rate limiting

## 14. Sci-Fi MMORPG Layer

Players can operate characters, ships, colonies and empires simultaneously.

Character roles include Space Marine, Engineer, Pilot, Psionic, Commander and Scientist.

Activities include exploration, boarding, raids, ground missions, diplomacy, fleet combat and empire management.

## 15. Galaxy Hierarchy

```text
Galaxy
 └─ Sector
    └─ Star System
       ├─ Planets
       ├─ Asteroid Fields
       ├─ Stations
       └─ Anomalies
```

## 16. Empire System

Empire attributes:

- Industry
- Science
- Economy
- Influence
- Logistics
- Intelligence
- Stability

Empire resources include Credits, Minerals, Gas, Energy, Data, Dark Matter, Antimatter, Nanites, Quantum Cores and Alien Artifacts.

## 17. Planet and Colony System

Planet attributes:

- Size
- Habitability
- Resources
- Defense
- Stability
- Population

Buildings include mining complexes, research labs, shipyards, trade hubs, defense grids, orbital cannons and habitat domes.

## 18. Fleet System

Fleet hierarchy:

```text
Fleet
 ├─ Flagship
 ├─ Capital Ships
 ├─ Cruisers
 ├─ Destroyers
 ├─ Frigates
 └─ Fighters
```

Fleet power:

```text
FleetPower = Sum(ShipCombatValue × CrewSkill × TechMultiplier)
```

Fleet statistics include speed, sensors, supply usage, morale and command capacity.

## 19. Ship Classes

Fighter, Corvette, Frigate, Destroyer, Cruiser, Battleship, Carrier, Dreadnought, Titan and Flagship.

## 20. Modular Ship Components

Weapons: Laser, Plasma, Railgun, Missile, Ion Cannon, Particle Beam.

Defense: Armor, Energy Shield, Point Defense.

Systems: Reactor, Sensors, Engines, AI Core, Targeting Computer.

The component system should support slots, power consumption, heat, mass, tech requirements, rarity and modifiers.

## 21. RTS Space Combat

```text
HitChance = Accuracy / (Accuracy + TargetEvasion)
Damage = WeaponDamage × TechMultiplier × CommanderBonus
FinalDamage = Damage × (1 - ShieldReduction) × (1 - ArmorReduction)
```

RTS commands include move, attack, defend, escort, intercept, retreat, focus fire, formation change, ability activation and boarding.

## 22. Turn-Based Tactical Combat

Used for boarding actions, colony invasions, base assaults, hero missions, dungeon encounters and tactical PvP.

Initiative:

```text
Initiative = Speed + TacticalSkill + EquipmentBonus
```

Actions:

- Move
- Attack
- Ability
- Overwatch
- Defend
- Use Item
- Interact
- Board
- Retreat

## 23. Research

Research branches:

- Physics
- Engineering
- Biotech
- Psionics
- Economics
- Military
- Computing

```text
ResearchTime = TechCost / (ScienceOutput × ScientistSkill)
```

Research unlocks ship components, buildings, weapons, defenses, abilities, colony technologies and strategic mechanics.

## 24. Diplomacy

Relations include Neutral, Trade Agreement, Non-Aggression Pact, Research Pact, Alliance, Vassal and War.

```text
DiplomaticScore = Influence + Reputation + TradeValue + SharedEnemies
```

## 25. Trade and Economy

```text
TradeIncome = TradeValue × RouteSecurity × DistanceModifier
```

Trade routes are affected by piracy, blockades, war zones, security, distance and faction control.

## 26. Galaxy Events

The persistent event system supports:

- Alien invasions
- Supernovae
- Rogue AI uprisings
- Ancient artifact discoveries
- Faction wars
- Economic crises
- Pirate campaigns
- Galactic disasters

Events can modify territory, resources, diplomacy, combat modifiers and victory progress.

## 27. AI Empire Logic

AI strategies include Expansionist, Defensive, Economic, Aggressive and Technological.

Priority model:

```text
Priority = ThreatLevel + ResourceNeed + ExpansionOpportunity + DiplomaticRisk
```

The AI should evaluate state, forecast consequences, select goals, issue actions and learn from outcomes through configurable utility weights.

## 28. Progression

Independent progression tracks:

```text
XP → Character Level
Fleet Victories → Commander Rank
Faction Missions → Reputation
Territory Control → Empire Rank
Research → Technology Tier
```

## 29. Victory Conditions

- Military Victory
- Economic Victory
- Technological Victory
- Diplomatic Victory
- Ascension Victory

Victory conditions may coexist in a persistent MMO season or be configured per scenario.

## 30. Server Architecture

```text
Client
 ↓
Gateway / Authentication / Rate Limiter
 ↓
Authoritative Game Server
 ├─ Character Service
 ├─ Combat Service
 ├─ AI / Encounter Service
 ├─ World / Instance Service
 ├─ Galaxy Service
 ├─ Fleet / Space Combat Service
 ├─ Empire Service
 ├─ Research / Diplomacy Service
 ├─ Tactical Service
 ├─ Social Service
 ├─ Economy Service
 └─ Persistence / Replication
 ↓
MySQL + Redis + Telemetry
```

## 31. Data-Driven Content

Core data assets/tables:

- CharacterData
- AttributeData
- StatData
- AbilityData
- EffectData
- ItemData
- WeaponData
- ArmorData
- EnemyData
- BossData
- EncounterData
- FactionData
- ShipData
- ComponentData
- PlanetData
- BuildingData
- FleetData
- TechData
- EventData
- DiplomacyData
- BalanceData

## 32. Master Power

```text
TotalPower = CharacterPower + FleetPower + EmpirePower + TechnologyPower + EconomicPower
```

This value is a strategic estimate and must not replace encounter-specific combat calculations.

## 33. Rust Module Mapping

The implementation should map this design into independent Rust modules with explicit ownership:

```text
attributes.rs       core character attributes
formulas.rs         deterministic formulas
resources.rs        resource pools
resistances.rs      resistance model
effects.rs          buffs/debuffs/CC
equipment.rs        equipment and item scaling
enemies.rs          enemy taxonomy and AI roles
boss.rs             boss phases and mechanics
encounters.rs       encounter definitions
combat.rs           combat resolution
abilities.rs        abilities and cooldowns
rpg.rs              classes, talents, quests
mmorpg.rs           sessions and MMO services
matchmaking.rs      PvP/PvE queues
fleet.rs            fleet state
ships.rs            ships/components
space_combat.rs     RTS space combat
galaxy.rs           galaxy hierarchy
planet.rs           colonies and buildings
empire.rs           empire state
research.rs         technology
diplomacy.rs        diplomacy
strategy.rs         strategic AI
turn_based.rs       tactical turns
galaxy_events.rs    persistent events
runtime.rs          authoritative ticking
server.rs           server facade
replication.rs      network state replication
telemetry.rs        metrics
rate_limit.rs       request protection
```

## 34. Production Requirements

The production implementation should provide deterministic simulation where practical, server authority, replayable combat events, explicit state ownership, configurable balance data, persistence boundaries, network-safe identifiers, observability, automated tests and versioned data schemas.

## 35. Implementation Rule

The engine is a framework, not a claim that every feature is already production-complete. New systems should be added as independently testable Rust modules and integrated through `lib.rs`, the authoritative runtime, persistence boundaries and server services.
