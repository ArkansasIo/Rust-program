# The New Game — D20 Fantasy Rulebook

## Purpose

This is the game's original D20 fantasy rulebook and systems specification. It uses familiar tabletop concepts—ability scores, proficiency, classes, levels, armor, hit points, saving throws, spells, conditions, and d20 resolution—while defining our own implementation, data, terminology, setting, encounters, progression, and digital-game behavior.

It is **not a reproduction of a commercial rulebook**. The Rust implementation in `src/dnd5e.rs`, `src/dnd5e_classes.rs`, and `src/dnd5e_spells.rs` is the authoritative starting point for the game's rules code.

## 1. Core Resolution

Most uncertain actions use a twenty-sided die plus a relevant modifier against a difficulty class (DC).

`total = d20 + ability_modifier + proficiency_bonus + situational_modifiers`

A result succeeds when `total >= DC`.

Natural 20 and natural 1 are tracked separately so the combat layer can apply game-specific critical rules.

### Difficulty guidance

| DC | Difficulty | Use |
|---:|---|---|
| 5 | Trivial | routine under pressure |
| 10 | Easy | ordinary trained task |
| 15 | Moderate | meaningful challenge |
| 20 | Hard | expert challenge |
| 25 | Severe | exceptional challenge |
| 30 | Legendary | world-class challenge |

## 2. Ability Scores

The six primary ability scores are Strength, Dexterity, Constitution, Intelligence, Wisdom, and Charisma.

`modifier = floor((score - 10) / 2)`

The engine exposes these as typed Rust values. Ability scores can be modified by ancestry, class, equipment, buffs, debuffs, talents, progression, and temporary effects.

## 3. Proficiency

Proficiency is a level-scaled bonus used by trained characters. The base progression is +2 at level 1 and increases by one point at levels 5, 9, 13, and 17.

Expertise doubles the proficiency contribution for systems that grant it.

## 4. Advantage and Disadvantage

When an action has advantage, the engine rolls two d20 values and uses the higher result. With disadvantage it uses the lower result. The game should combine multiple sources into one state rather than stack unlimited extra dice.

## 5. Character Creation

A character contains:

- identity and name
- ancestry/race
- class
- level and experience
- alignment/personality metadata
- six ability scores
- hit die
- hit points
- armor class
- movement speed
- proficiencies
- conditions
- equipment
- inventory
- spells and spell slots
- talents and subclass features
- quests and progression state

### Starting classes

The first release supports twelve archetypal classes:

Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Warlock, and Wizard.

Their implementation lives in `dnd5e_classes.rs` and is data-driven so the game can later add custom classes without rewriting the combat engine.

### Starting ancestries

The engine supports Human, Elf, Dwarf, Halfling, Orc, Tiefling, Dragonkin, Gnome, Half-Elf, Half-Orc, Aasimar, and Custom as initial identifiers. The project's own setting will define the final lore, bonuses, subtypes, cultures, and playable variants.

## 6. Combat

A combat turn is represented by available action resources rather than a hard-coded animation sequence.

Resources include:

- movement
- action
- bonus action
- reaction
- free interaction

The existing battle engine remains the authoritative simulation layer. D20 rules feed attack checks, saving throws, spell attacks, damage, healing, conditions, and ability checks into that simulation.

### Attack check

`attack_total = d20 + relevant_ability_modifier + proficiency_if_trained + bonuses`

The attack succeeds when it reaches the target's armor class.

### Armor Class

The base game implementation provides a composable calculation:

`AC = base + dexterity_modifier + armor_bonus + shield_bonus`

Special armor rules can override or cap the Dexterity contribution through equipment definitions.

### Hit Points

Level-one HP uses the class hit die plus Constitution modifier. Later levels use the selected average or a rolled value plus Constitution modifier.

`HP = first_level_value + (level - 1) * per_level_value`

The runtime clamps HP to at least one for a living newly-created character.

## 7. Damage and Resistance

The game's damage taxonomy includes:

- bludgeoning
- piercing
- slashing
- acid
- cold
- fire
- force
- lightning
- necrotic
- poison
- psychic
- radiant
- thunder

The existing resistance system should process immunity, resistance, vulnerability, flat mitigation, shields, and temporary protection before final HP loss.

## 8. Conditions

The condition system provides typed states such as blinded, charmed, deafened, frightened, grappled, incapacitated, invisible, paralyzed, petrified, poisoned, prone, restrained, stunned, unconscious, and exhausted.

Conditions are runtime objects rather than UI-only flags. They must be serialized for persistence and replicated to clients when visible.

## 9. Saving Throws

Saving throws use one of the six abilities plus proficiency when the character is trained in that saving throw.

`save_total = d20 + ability_modifier + proficiency + modifiers`

The target DC is supplied by the ability, spell, trap, monster, environment, or encounter definition.

## 10. Spellcasting

The spell subsystem provides:

- spell levels 0–9
- schools of magic
- casting time
- range
- damage type
- damage dice
- concentration state
- spell attack modifiers
- spell save DCs
- spell slots
- recovery
- class-specific casting models

The initial catalog in `data/dnd5e_spells.csv` contains original game spell names and descriptions. More spells should be added as data rather than hard-coded into combat logic.

## 11. Monsters and NPCs

Monsters are built from the same combat primitives as player characters but can expose custom AI, resistances, legendary behavior, boss phases, lair effects, summons, and scripted encounter mechanics.

Recommended monster definition fields:

`id, name, level, role, size, armor_class, max_hp, speed, threat_weight, resistances, abilities, loot_table`

The existing AI archetype, enemy, boss, threat, encounter, and loot systems form the foundation for this layer.

## 12. Bosses

Bosses use the existing phase runtime. A boss may transition at health thresholds and change damage, mechanics, summoned adds, arena hazards, or ultimate behavior.

Phase definitions should remain data-driven so dungeon and raid designers can author encounters without changing Rust code.

## 13. Equipment

Equipment is integrated with the existing inventory/equipment system. Recommended slots:

- head
- shoulders
- chest
- hands
- legs
- feet
- main hand
- off hand
- ranged/focus
- ring 1
- ring 2
- amulet
- relic

Equipment can modify ability scores, defenses, movement, resources, spell power, critical chance, resistances, and custom RPG attributes.

## 14. Progression

The game uses level progression from 1 through 20 for the initial D20 rules layer. MMO progression can continue beyond level 20 through separate systems such as:

- mastery levels
- prestige ranks
- talent nodes
- equipment tiers
- reputation
- professions
- account progression
- seasonal progression

This keeps tabletop-style class progression separate from long-term MMO progression.

## 15. Rest and Recovery

The game supports short recovery and long recovery concepts. Exact durations and resource restoration are server-configurable so the MMO can balance pacing independently from tabletop play.

## 16. Skills and Proficiencies

Skill checks are ability checks with optional proficiency or expertise. The engine should maintain a typed registry of skills so future expansions can add setting-specific skills.

Initial skill families:

- Acrobatics — Dexterity
- Animal Handling — Wisdom
- Arcana — Intelligence
- Athletics — Strength
- Deception — Charisma
- History — Intelligence
- Insight — Wisdom
- Intimidation — Charisma
- Investigation — Intelligence
- Medicine — Wisdom
- Nature — Intelligence
- Perception — Wisdom
- Performance — Charisma
- Persuasion — Charisma
- Religion — Intelligence
- Sleight of Hand — Dexterity
- Stealth — Dexterity
- Survival — Wisdom

## 17. Dungeon and MMO Integration

The D20 rules are one layer of the larger game. They must integrate with:

`Input -> Command -> Validation -> D20 Resolution -> Combat/Ability -> Effects -> Threat -> Loot -> Quest Progress -> Replication -> Persistence`

The server is authoritative. Clients request actions; they do not authoritatively decide hit results, damage, loot, XP, or progression.

## 18. Multiplayer

Party, guild, matchmaking, dungeon, raid, PvP, chat, trading, economy, and social systems use the same character and combat model.

All authoritative state must be serializable and versionable for persistence and replication.

## 19. AI

AI archetypes can use D20-style checks when appropriate but should not be forced through tabletop rules for every decision. The AI layer can use threat, distance, cooldowns, role, target priority, encounter phase, and tactical state.

## 20. World Design

The new game combines medieval fantasy with the project's broader RPG/MMORPG architecture. The world is organized into:

`Universe -> World -> Continent -> Region -> Zone -> Subzone -> Dungeon -> Encounter -> Room`

Each layer can contain quests, NPCs, enemies, resources, events, treasures, bosses, and dynamic world state.

## 21. Data-Driven Content

Rules code should stay stable while designers edit CSV/TOML/JSON content.

Current D20 data files include:

- `data/dnd5e_classes.csv`
- `data/dnd5e_conditions.csv`
- `data/dnd5e_spells.csv`
- `data/core_attributes.csv`
- `data/enemies.csv`
- `data/boss_phase_templates.csv`
- `data/ai_archetypes.csv`

The content loader validates required columns before data reaches runtime.

## 22. Implementation Roadmap

### Phase A — Core rules

- [x] ability scores
- [x] modifiers
- [x] proficiency
- [x] d20 resolution
- [x] advantage/disadvantage
- [x] classes
- [x] races
- [x] conditions
- [x] HP/AC/initiative
- [x] spell slots

### Phase B — Full character system

- [ ] level-by-level class features
- [ ] subclasses
- [ ] feats/talents
- [ ] backgrounds
- [ ] skills/proficiency registry
- [ ] complete equipment definitions
- [ ] spell preparation/known-spell management

### Phase C — Encounter system

- [ ] monster stat blocks
- [ ] encounter budgets
- [ ] traps
- [ ] environmental hazards
- [ ] boss scripting
- [ ] legendary actions/custom reactions

### Phase D — MMO layer

- [ ] authoritative combat service
- [ ] party/raid rules
- [ ] dungeon instances
- [ ] PvP normalization
- [ ] persistence
- [ ] replication
- [ ] telemetry
- [ ] anti-cheat validation

### Phase E — World

- [ ] campaign setting
- [ ] factions
- [ ] continents and zones
- [ ] towns and capitals
- [ ] dungeons and raids
- [ ] world bosses
- [ ] procedural encounters
- [ ] live events

## 23. Source of Truth

Rust modules are the executable specification. CSV/TOML content is the designer-facing specification. This book documents the contract between them.

When a rule changes, update the implementation, content data, tests, and this document together.
