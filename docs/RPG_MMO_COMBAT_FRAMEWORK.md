# RPG / MMORPG Combat, Stats, Equipment, Enemy, Boss & Encounter Framework

## 1. Purpose

This document defines the production-oriented RPG/MMORPG gameplay model used by the Rust framework. It covers attributes, sub-attributes, combat statistics, resources, damage and resistance, buffs/debuffs, equipment, enemies, mobs, elites, bosses, encounters, PvE/PvP normalization, scaling, and data-driven balance.

The model is engine-agnostic and designed for a server-authoritative Rust simulation. It can also map cleanly to Unreal Engine 5 Blueprint/C++ data assets and data tables.

## 2. Layered Stat Architecture

```text
Character / Unit
  ├── Core Attributes
  │   ├── STR
  │   ├── DEX
  │   ├── CON
  │   ├── INT
  │   ├── WIS
  │   ├── VIT
  │   ├── SPI
  │   ├── LCK
  │   ├── WIL
  │   └── CHA
  ├── Sub-Attributes
  ├── Combat Stats
  ├── Resources
  ├── Resistances
  ├── Equipment
  ├── Talents / Skills
  ├── Buffs
  ├── Debuffs
  └── Context Rules
       ├── PvE
       ├── PvP
       ├── Dungeon
       ├── Raid
       └── World Event
```

A final combat value is always contextual. Item level, raw power, and practical effectiveness are separate concepts.

## 3. Core Attributes

| Attribute | Primary purpose |
|---|---|
| STR | Physical power, melee damage, carry capacity |
| DEX | Accuracy, attack speed, evasion, critical chance |
| CON | Health, stamina, physical resistance |
| INT | Spell power, mana, magical scaling |
| WIS | Healing power, mana sustain, status resistance |
| VIT | Maximum HP and survivability |
| SPI | Resource regeneration and aura strength |
| LCK | Proc chance, loot variance and luck-based effects |
| WIL | Crowd-control resistance and resolve |
| CHA | Threat control, NPC interaction and leadership |

## 4. Sub-Attributes

### Physical

- Physical Power
- Weapon Handling
- Accuracy
- Evasion
- Attack Speed
- Armor Penetration
- Critical Chance
- Critical Damage
- Recovery Speed

### Magical

- Spell Power
- Elemental Mastery
- Cast Speed
- Magic Penetration
- Mana Efficiency
- Overcharge
- Healing Power
- Status Power

### Defensive

- Maximum HP
- HP Regeneration
- Armor
- Magic Resistance
- Damage Reduction
- Shield Strength
- Block Chance
- Block Value
- Parry Chance
- Tenacity
- Poise

### Utility

- Movement Speed
- Cooldown Reduction
- Resource Regeneration
- Threat Generation
- Threat Reduction
- Detection
- Stealth
- Command Capacity

## 5. Combat Statistics

### Offensive

- Base Damage
- Skill Damage
- Attack Power
- Spell Power
- Critical Chance
- Critical Damage
- Combo Multiplier
- Backstab Multiplier
- Weak-Point Damage
- Execute Damage

### Defensive

- Armor
- Magic Resistance
- Mitigation
- Guard Strength
- Dodge
- Block
- Parry
- Shield Capacity
- Shield Regeneration

### Speed / Flow

- Attack Speed
- Cast Speed
- Cooldown Reduction
- Global Cooldown
- Movement Speed
- Initiative / Turn Priority

## 6. Resources

Standard pools:

- HP
- MP
- Stamina

Optional class resources:

- Energy
- Rage
- Focus
- Faith
- Aether
- Heat
- Sanity

Every resource should define maximum value, current value, regeneration, regeneration modifiers, cost rules, and combat-state restrictions.

## 7. Damage Types

The framework supports:

- Physical
- Fire
- Ice
- Lightning
- Earth
- Wind
- Water
- Light
- Dark
- Arcane
- Poison
- Bleed
- Void
- Chaos
- True Damage

## 8. Resistance Pipeline

```text
Raw Damage
  -> Flat Reduction
  -> Penetration
  -> Percentage Resistance
  -> Vulnerability
  -> Absorption / Shield
  -> Damage Reduction
  -> Final Damage
```

Resistance can expose flat values, percentages, absorption amounts, immunity flags, vulnerability multipliers, and penetration interactions.

## 9. Buff System

### Offensive buffs

- Power Surge
- Berserk
- Arcane Amplify
- Precision
- Elemental Infusion
- Critical Focus
- Execute Window

### Defensive buffs

- Shielded
- Fortified
- Regeneration
- Damage Absorption
- Invulnerability
- Barrier
- Resistance Ward

### Utility buffs

- Haste
- Stealth
- Invisibility
- Flight
- True Sight
- Phase Shift
- Movement Boost

A buff should define source, category, duration, stacking rule, magnitude, periodic tick, dispel policy, and PvP/PvE modifier.

## 10. Debuff System

### Crowd control

- Stun
- Freeze
- Root
- Silence
- Fear
- Charm
- Sleep
- Knockback
- Knockdown

### Damage over time

- Burn
- Poison
- Bleed
- Corruption
- Frostbite
- Shock

### Attribute reduction

- Weakness: STR reduction
- Frailty: CON reduction
- Hex: INT reduction
- Slow: movement/tempo reduction
- Cripple: DEX reduction
- Curse: broad-stat reduction

## 11. Crowd-Control Diminishing Returns

Default PvP/boss sequence:

```text
First application  = 100%
Second application = 50%
Third application  = 25%
Fourth application = temporary immunity
```

The DR bucket should be keyed by target, effect family, and configured time window.

## 12. Equipment

Recommended slots:

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

Equipment supports rarity, required level, stat rolls, sockets, set bonuses, procs, cosmetic metadata, PvE stats, PvP stats, and scaling rules.

Rarity progression:

```text
Common -> Uncommon -> Rare -> Epic -> Legendary -> Mythic
```

## 13. Power and Value Formulas

### Stat Power Units

```text
StatValuePU = RawStat * Weight * ScalingCurve
```

Example weights:

```text
STR / DEX / INT = 1.0
Crit Chance      = 2.5
Attack Speed     = 2.0
Armor            = 0.75
Resilience       = 3.0
```

### Soft cap

```text
EffectiveStat = Raw / (Raw + SoftCap)
```

### Diminishing returns

```text
Effective = Cap * (1 - e^(-Raw / K))
```

### Armor mitigation

```text
Mitigation = Armor / (Armor + K * AttackerLevel)
```

### Example character formulas

```text
MaxHP = VIT * 15 + CON * 10 + GearHP
PhysicalDamage = STR * 2.5 * WeaponMultiplier
CritChance = DEX * 0.04 + GearCrit
DamageTaken = IncomingDamage * (1 - DamageMitigation)
```

## 14. PvE / PvP Context Scaling

Recommended baseline conversion:

| Stat | PvE | PvP |
|---|---:|---:|
| Damage | 100% | 70% |
| Crit Chance | 100% | 50% |
| Crit Damage | 100% | 60% |
| Healing | 100% | 65% |
| CDR | 100% | 50% |
| CC Duration | 100% | 40% |

Hard caps should be configurable rather than hard-coded into individual abilities.

## 15. Enemy Taxonomy

### Common Mob

Purpose: farming, quests, XP, world population.

Typical power: 60–80% of player baseline.

Characteristics:

- Low HP/damage
- 1–2 abilities
- Minimal immunities
- Simple AI

### Veteran / Strong Mob

Typical power: 120–150% of player baseline.

Characteristics:

- Higher durability
- 3–5 abilities
- Partial resistances
- Better positioning logic

### Elite

Typical power: 200–300% of baseline.

Characteristics:

- Mini-boss behavior
- Strong CC resistance
- Enrage timer
- More complex rotations
- Improved loot

### Rare / Named Enemy

Characteristics:

- Conditional spawn
- Unique name and identity
- Unique loot table
- Special modifiers
- Custom ability package

### Boss

Typical power: 500–5000% of baseline depending on encounter scope.

Characteristics:

- Phase system
- Immunity layers
- Mechanics and telegraphs
- Adds
- Enrage timer
- Unique rewards

## 16. Boss Classification

Supported boss classes:

- Dungeon Boss
- World Boss
- Raid Boss
- Event Boss
- Story Boss
- Mythic Boss
- Secret Boss

## 17. Boss Phase System

Phase triggers can include:

- HP thresholds: 75%, 50%, 25%
- Elapsed time
- Player death count
- Objective completion
- Environmental state

A phase can change:

- Ability set
- Arena geometry
- Adds
- Damage types
- Resistances
- Movement rules
- Enrage state
- Loot eligibility

Example:

```text
Phase 1 -> Standard combat
Phase 2 -> Enrage + adds
Phase 3 -> Environmental hazards
Phase 4 -> Ultimate ability
```

## 18. AI Archetypes

- Brute: slow, durable, high damage
- Assassin: stealth, burst, target isolation
- Caster: ranged magical damage
- Controller: crowd control and area denial
- Summoner: creates adds/minions
- Support: heals and buffs allies
- Tank: threat generation and mitigation
- Sniper: long-range precision

AI decisions should evaluate threat, distance, health, resources, cooldowns, target role, encounter phase, and tactical objectives.

## 19. Encounter Scaling

Boss health:

```text
BossHP = Base * (1 + LevelDiff * 0.18) * PartyMult * DifficultyMult
PartyMult = 1 + (Players - 1) * 0.65
```

Encounter power budget:

```text
EncounterPowerBudget = Sum(PlayerPower) * DifficultyFactor
```

Boss mechanics should remain within the encounter budget unless they provide clear telegraphs, counterplay, or limited frequency.

## 20. Mob Spawning

Spawn definitions should contain:

```text
spawn_id
zone_id
archetype
level_range
population_cap
respawn_time
spawn_radius
patrol_radius
rare_spawn_chance
loot_table
faction
aggro_radius
leash_radius
```

Spawn systems should support deterministic seeds, population caps, respawn waves, event modifiers, and server-side validation.

## 21. Threat / Aggro

Threat is a first-class combat value.

```text
Threat = DamageThreat + HealingThreat + TauntThreat + RoleModifier
```

Threat tables should support target ranking, forced target effects, threat transfer, threat reduction, leash resets, and encounter-specific multipliers.

## 22. Loot

Loot can be generated from:

- Enemy rank
- Boss identity
- Difficulty
- Player contribution
- Luck
- Loot table
- Event multiplier
- Party rules

Competitive PvP should avoid power-bearing RNG whenever possible.

## 23. Encounter Lifecycle

```text
Spawn
 -> Detect Players
 -> Acquire Threat
 -> Combat
 -> Phase Evaluation
 -> Ability Selection
 -> Damage / Effects
 -> Adds / Objectives
 -> Death / Wipe
 -> Rewards
 -> Cleanup
 -> Respawn / Reset
```

## 24. MMO Services

The server architecture should separate:

- Authentication
- Account service
- Character service
- Inventory/equipment
- Combat simulation
- AI and threat
- Encounter instances
- Dungeon/raid service
- Matchmaking
- Guild/social systems
- Chat
- Economy/trading/auction
- Persistence
- Replication
- Telemetry
- Rate limiting
- Sharding

## 25. Data-Driven Design

Recommended content objects:

```text
CoreAttributes
DerivedStats
CombatStats
ResourceDefinitions
ResistanceDefinitions
BuffDefinitions
DebuffDefinitions
ItemDefinitions
WeaponDefinitions
ArmorDefinitions
EnemyDefinitions
BossDefinitions
AbilityDefinitions
EncounterDefinitions
LootTables
ScalingRules
PvPRules
```

Seed data is stored in `data/` and runtime rules are stored in `config/`.

## 26. Server Authority

Clients submit intents. The server validates and resolves the authoritative result.

```text
Client Input
 -> Intent
 -> Validation
 -> Ability / Combat System
 -> Stat Snapshot
 -> Effect Resolution
 -> Damage / Resource Change
 -> Event
 -> Replication
```

The client must not be trusted to submit final damage, final position, cooldown completion, loot ownership, or inventory state.

## 27. Telemetry

Track at minimum:

- Damage by source/type
- Healing
- Deaths
- Boss phase duration
- Encounter wipes
- Time-to-kill
- Ability usage
- CC duration
- Proc frequency
- Item power distribution
- PvP burst windows
- Economy rewards

Telemetry feeds live balance and seasonal tuning.

## 28. Safety and Balance Rules

- HP scaling should normally exceed damage scaling in PvE.
- CC requires diminishing returns.
- PvP should reduce burst and healing extremes.
- Competitive RNG should be limited.
- Every major stat needs a cap or diminishing-return curve.
- Boss mechanics require counterplay.
- Server authority controls combat outcomes.
- Balance formulas are centralized for testing and telemetry.

## 29. Integration with the Rust Framework

The existing Rust modules provide the implementation surface for this design:

```text
attributes.rs       -> attribute model
stats.rs            -> runtime combat stats
resources.rs        -> resource pools
resistances.rs      -> resistance model
effects.rs          -> effects
status.rs           -> status management
combat.rs           -> damage and combat events
abilities.rs        -> abilities/cooldowns
equipment.rs        -> equipment rules
enemies.rs          -> enemy definitions
boss.rs             -> boss/phase definitions
encounters.rs       -> encounters
loot.rs             -> loot generation
threat.rs           -> aggro/threat
ai.rs               -> AI controllers
formulas.rs         -> centralized formulas
runtime.rs          -> simulation tick
server.rs           -> authoritative server facade
replication.rs      -> state replication
telemetry.rs        -> metrics
rate_limit.rs       -> request limiting
matchmaking.rs      -> PvP/PvE queues
```

## 30. Extension to Sci-Fi 4X / RTS / Turn-Based Systems

The RPG character layer can connect to the strategic layer:

```text
Character Power
 + Fleet Power
 + Empire Power
 + Technology Power
 + Economic Power
 = Total Strategic Power
```

This permits ground characters, ship crews, commanders, fleets, colonies, research, diplomacy, and tactical battles to participate in one persistent simulation.

## 31. Recommended Future Content Modules

- Mounts and vehicles
- Companion/pet system
- Talent trees
- Class/subclass system
- Prestige and ascension
- World events
- Dynamic dungeon generation
- Raid lockouts
- Seasonal modifiers
- PvP rating/MMR
- Territory control
- Guild technology
- Faction reputation
- Crafting specializations
- Item sockets and runes
- Enchantments
- Transmog/cosmetics
- Achievement system
- Replay/event audit logs

## 32. Reference

The framework specification is based on the supplied RPG/MMORPG combat and equipment requirements, including the attribute, sub-stat, resource, buff/debuff, enemy, boss, and Sci-Fi 4X/RTS/turn-based extensions. The supplied material explicitly calls for a data-driven, server-authoritative design and lists the RPG and strategic layers as interoperating systems.
