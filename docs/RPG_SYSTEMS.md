# RPG Systems Specification

## Character
- Character identity and account ownership
- Character classes
- Attributes and combat resources
- Level and experience progression
- Skills and abilities
- Talent ranks
- Equipment and inventory
- Status effects
- Achievements

## Combat
Combat is built around deterministic ticks and explicit events. Damage types, attack kinds, mitigation, critical hits, cooldowns, threat and death are modeled independently so they can be reused by PvE and PvP.

## Content
Zones define level bands and positions. Spawn points define population and respawn timing. Dungeons and raids define party limits, bosses and loot-table references. NPCs and dialogue provide extensible interaction data.

## Economy
Wallets, crafting recipes, auction listings, player trading and loot tables provide the foundation for a persistent game economy.

## Design rules
1. Keep game rules data-driven where practical.
2. Avoid UI assumptions inside simulation modules.
3. Keep authoritative state on the server.
4. Emit events for cross-system effects instead of tightly coupling systems.
5. Make deterministic calculations testable without a network or database.
