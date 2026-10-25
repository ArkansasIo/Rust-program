# THE NEW GAME — D20 FANTASY CORE RULES

## Book I — Character

Characters are built from identity, ancestry, class, subclass, background, ability scores, skills, feats, equipment, spells, progression, and MMO metadata. The executable character foundation is in `src/dnd5e.rs` and `src/dnd5e_character.rs`.

### Ability scores

Strength, Dexterity, Constitution, Intelligence, Wisdom, and Charisma use the modifier formula `floor((score - 10) / 2)`. The engine keeps the six scores typed and serializable.

### Proficiency

The initial progression is +2 at level 1 and increases at levels 5, 9, 13, and 17. Expertise doubles the proficiency contribution.

### Skills

The skill registry contains Acrobatics, Animal Handling, Arcana, Athletics, Deception, History, Insight, Intimidation, Investigation, Medicine, Nature, Perception, Performance, Persuasion, Religion, Sleight of Hand, Stealth, and Survival.

## Book II — Resolution

Uncertain actions resolve as `d20 + modifiers >= DC`. The engine records the natural d20 separately from the total so game-specific critical rules can be layered on top. Advantage uses the higher of two rolls; disadvantage uses the lower.

Difficulty bands: 5 trivial, 10 easy, 15 moderate, 20 hard, 25 severe, 30 legendary.

## Book III — Combat

A turn exposes movement, action, bonus action, reaction, and free interaction resources. The server validates resource use before executing commands.

Attack resolution is `d20 + ability modifier + proficiency when trained + bonuses` against Armor Class. Damage is processed through immunity, resistance, vulnerability, shields, and final HP loss.

The D20 combat resolver is `src/dnd5e_combat.rs`; the general battle engine remains authoritative for the actual simulation.

## Book IV — Magic

Spells have level, school, casting time, range, damage, concentration, and runtime availability. Spell slots are tracked per level. Prepared and known spells are distinct collections. Concentration is a single active runtime state and can be broken by game events.

`src/dnd5e_spells.rs` defines spell data structures. `src/dnd5e_spell_runtime.rs` owns casting state.

## Book V — Conditions and Effects

Conditions are typed runtime state. The existing status/effects systems are the integration point for duration, periodic effects, control restrictions, cleansing, immunity, and replication.

## Book VI — Equipment

Equipment occupies explicit slots and can contribute armor, ability modifiers, damage, resistances, and value. The equipment layer is designed to merge with the existing inventory/equipment modules.

## Book VII — Classes and Subclasses

The first class layer contains twelve archetypes: Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Warlock, Wizard.

Subclasses are separate data definitions so the class core remains stable. The initial game includes original subclass hooks such as Berserker Path, Battle Mastery, Arcane School, and Shadow Path.

## Book VIII — Monsters

Monster stat blocks use the same abilities and damage taxonomy as characters. A monster may additionally define resistances, immunities, actions, AI role, loot table, threat weight, boss behavior, and scripted encounter mechanics.

Roles include Brute, Tank, Assassin, Caster, Controller, Support, Summoner, Skirmisher, Artillery, and Boss.

## Book IX — Encounters

Encounters are composed from slots containing monster IDs and counts. Difficulty, party size, level range, arena, rewards, traps, and hazards are data-driven. Encounter budgets can prevent content authors from exceeding intended difficulty.

## Book X — Traps and Hazards

Traps define detection DC, disarm DC, saving-throw DC, damage, trigger behavior, and reset behavior. Hazards define area, duration, save DC, damage, and damage type.

## Book XI — Loot

Reward tables contain weighted entries, rarity, item IDs, and level ranges. Existing deterministic loot functionality can consume these definitions. D20 reward selection is a content layer, not a separate random-number implementation.

## Book XII — Rest and Progression

Short and long recovery are represented as explicit game concepts. MMO pacing may configure how much health, spell resources, cooldowns, and conditions are restored.

Levels 1–20 form the core D20 character progression. Beyond that, the MMO can use mastery, prestige, account progression, gear tiers, professions, reputation, and seasonal systems.

## Book XIII — World

The campaign world hierarchy is:

`Universe -> World -> Continent -> Region -> Zone -> Subzone -> Dungeon -> Encounter -> Room`

Each node can contain NPCs, quests, enemies, resources, events, treasures, bosses, and dynamic state.

## Book XIV — MMO Rules

The server is authoritative. The client submits commands. The server validates ownership, cooldowns, action resources, range, line of sight, target state, D20 rolls, damage, loot, XP, and persistence. Replication then publishes the resulting state.

Core flow:

`Input -> Command -> Validation -> D20 Resolution -> Combat/Ability -> Effects -> Threat -> Loot -> Quest Progress -> Replication -> Persistence`

Party, guild, matchmaking, dungeon, raid, PvP, trade, economy, chat, and social services use the same character model.

## Book XV — AI

AI may use D20 checks where appropriate but is not forced to simulate tabletop behavior for every decision. AI combines archetype, threat, distance, cooldowns, target priority, role, encounter phase, and world state.

## Book XVI — Content Pipeline

Designers author CSV/TOML/JSON. Rust modules validate and execute the resulting definitions. Required IDs must be unique. Enumerations must resolve to known runtime values. Numeric ranges must be validated before content enters authoritative state.

Current D20 data files:

- `data/dnd5e_classes.csv`
- `data/dnd5e_conditions.csv`
- `data/dnd5e_spells.csv`
- `data/dnd5e_skills.csv`
- `data/dnd5e_races.csv`
- `data/dnd5e_backgrounds.csv`

## Book XVII — Digital Game Extensions

The game is not restricted to tabletop encounters. It extends the D20 foundation into persistent characters, procedural dungeons, raids, world bosses, PvP, crafting, economy, guilds, space/strategy systems, telemetry, replication, and server-authoritative simulation.

## Book XVIII — Source of Truth

Executable rules live in Rust. Designer content lives in data files. This book defines the contract. When a rule changes, update implementation, data, tests, and documentation together.

## Legal / Content Boundary

This project uses an original fantasy setting and original game-facing content. It may use compatible open rules concepts from the applicable D&D System Reference Document. It does not reproduce proprietary rulebook text. Wizards of the Coast states that SRD 5.1 is available under Creative Commons Attribution 4.0; creators should follow the applicable SRD attribution and licensing requirements when using SRD material. citeturn0search0turn0search1
