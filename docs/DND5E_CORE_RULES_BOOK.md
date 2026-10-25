# THE NEW GAME — D20 FANTASY CORE RULES

## Book I — Character
Characters are built from identity, ancestry, class, subclass, background, ability scores, skills, feats, equipment, spells, progression, and MMO metadata. The executable character foundation is in `src/dnd5e.rs` and `src/dnd5e_character.rs`.

## Book II — Resolution
Uncertain actions resolve as `d20 + modifiers >= DC`. Natural d20 values are retained separately for game-specific critical handling. Advantage uses the higher of two rolls; disadvantage uses the lower. Difficulty bands are 5 trivial, 10 easy, 15 moderate, 20 hard, 25 severe, and 30 legendary.

## Book III — Combat
A turn exposes movement, action, bonus action, reaction, and free interaction resources. Attack resolution is `d20 + ability modifier + proficiency when trained + bonuses` against Armor Class. Damage passes through immunity, resistance, vulnerability, shields, and final HP loss.

## Book IV — Magic
Spells define level, school, casting time, range, damage, concentration, and runtime availability. Spell slots are tracked per level. Prepared and known spells are separate collections. Concentration is an explicit runtime state.

## Book V — Conditions and Effects
Conditions are typed runtime state. The existing status/effects systems provide duration, periodic effects, control restrictions, cleansing, immunity, and replication hooks.

## Book VI — Equipment
Equipment occupies explicit slots and can contribute armor, ability modifiers, damage, resistances, and value. It integrates with the existing inventory/equipment system.

## Book VII — Classes and Subclasses
The first class layer contains Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Warlock, and Wizard. Subclasses are separate definitions so class cores remain stable. The initial project includes original subclass hooks such as Berserker Path, Battle Mastery, Arcane School, and Shadow Path.

## Book VIII — Monsters
Monster stat blocks use the same abilities and damage taxonomy as characters. A monster may additionally define resistances, immunities, actions, AI role, loot table, threat weight, boss behavior, and scripted mechanics. Roles include Brute, Tank, Assassin, Caster, Controller, Support, Summoner, Skirmisher, Artillery, and Boss.

## Book IX — Encounters
Encounters are composed from monster slots. Difficulty, party size, level range, arena, rewards, traps, hazards, and encounter budgets are data-driven.

## Book X — Traps and Hazards
Traps define detection DC, disarm DC, saving-throw DC, damage, trigger behavior, and reset behavior. Hazards define area, duration, save DC, damage, and damage type.

## Book XI — Loot
Reward tables contain weighted entries, rarity, item IDs, and level ranges. The existing deterministic loot layer can consume these definitions.

## Book XII — Rest and Progression
Short and long recovery are explicit concepts. MMO pacing can configure health, spell resources, cooldowns, and condition restoration. Levels 1–20 form the core D20 progression; MMO systems can continue through mastery, prestige, account progression, gear tiers, professions, reputation, and seasons.

## Book XIII — World
`Universe -> World -> Continent -> Region -> Zone -> Subzone -> Dungeon -> Encounter -> Room`

Each node can contain NPCs, quests, enemies, resources, events, treasures, bosses, and dynamic state.

## Book XIV — MMO Rules
The server is authoritative. Clients submit commands. The server validates ownership, cooldowns, action resources, range, line of sight, targets, D20 results, damage, loot, XP, and persistence before replication.

Core flow:
`Input -> Command -> Validation -> D20 Resolution -> Combat/Ability -> Effects -> Threat -> Loot -> Quest Progress -> Replication -> Persistence`

## Book XV — AI
AI combines archetype, threat, distance, cooldowns, target priority, role, encounter phase, and world state. D20 checks may be used for uncertain AI actions but are not required for every decision.

## Book XVI — Content Pipeline
Designers author CSV/TOML/JSON. Rust validates and executes the definitions. Required IDs must be unique, enum values must resolve, and numeric ranges must be checked before content enters authoritative state.

Current D20 data files include `data/dnd5e_classes.csv`, `data/dnd5e_conditions.csv`, `data/dnd5e_spells.csv`, `data/dnd5e_skills.csv`, `data/dnd5e_races.csv`, and `data/dnd5e_backgrounds.csv`.

## Book XVII — Digital Game Extensions
The D20 layer extends into persistent characters, procedural dungeons, raids, world bosses, PvP, crafting, economy, guilds, strategy systems, telemetry, replication, and server-authoritative simulation.

## Book XVIII — Source of Truth
Executable rules live in Rust. Designer content lives in data files. This book documents their contract. When a rule changes, update implementation, content, tests, and documentation together.

## Legal / Content Boundary
This project uses an original fantasy setting and original game-facing content. It does not reproduce proprietary rulebook text. If SRD material is used, the project should follow the applicable System Reference Document licensing and attribution requirements. Wizards of the Coast publishes SRD material under Creative Commons terms.
