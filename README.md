# Rust Battle Engine

A modular Rust combat and battle-game engine for RPG/MMORPG projects. The repository has been repurposed from the previous mathematics/environment API into a gameplay foundation.

## Engine modules

- `core` — entity IDs, teams, ticks, vectors.
- `entities` — combatant state and runtime actors.
- `stats` — RPG attributes, resources, armor, resistance, critical hits, attack and movement stats.
- `combat` — damage calculation, mitigation, critical attacks, damage types, death handling.
- `abilities` — skills, range checks, resource costs, cooldowns.
- `ai` — target selection and basic idle/chase/attack state machine.
- `battle` — deterministic battle loop and victory detection.
- `engine` — top-level battle orchestration.
- `world` — entity storage, ticks, and spatial queries.
- `physics` — movement and range primitives.
- `items` / `inventory` — item definitions, weapons, armor, consumables, equipment.
- `loot` — deterministic weighted loot tables.
- `progression` — XP, levels, and skill points.
- `events` — combat/game event bus.

## Example

```bash
cargo run
cargo test
```

The executable runs a small Knight-versus-Goblin battle using the same public engine APIs intended for larger games.

## Combat pipeline

`Input/AI -> Target Selection -> Range Check -> Ability/Attack -> Damage Calculation -> Mitigation -> Resource Update -> Death -> Event Bus -> Battle Result`

The architecture is intentionally data-driven so abilities, items, stats, enemies, encounters, and battle rules can later be loaded from JSON, TOML, databases, or a game-server API.

## Planned engine layers

The next engine layers are behavior-tree AI, status effects and crowd control, aggro/threat, equipment stat modifiers, quests, crafting, dungeon/raid encounters, PvP rules, deterministic replays, save/load, networking, and dedicated-server simulation.

## License

MIT
