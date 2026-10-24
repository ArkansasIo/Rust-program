# Master Game Design

## Purpose

This document consolidates the RPG/MMORPG combat framework and the Sci-Fi MMORPG + 4X + RTS + turn-based strategy framework supplied for this project.

## Gameplay layers

### Character
Levels, classes, attributes, sub-attributes, skills, talents, equipment, resources, effects, quests, achievements, factions, reputation and progression.

### Combat
Action or turn-based combat, damage types, resistances, mitigation, critical hits, healing, shields, crowd control, threat, PvE scaling and PvP normalization.

### PvE
Common mobs, veterans, elites, rare/named enemies, dungeon bosses, raid bosses, world bosses, event bosses, story bosses, mythic bosses and secret bosses.

### MMORPG
Persistent characters, sessions, parties, guilds, chat, friends, mail, matchmaking, instances, world shards, economy, crafting, auction, trade, persistence and replication.

### 4X
Explore, Expand, Exploit and Exterminate across a persistent galaxy containing sectors, systems, planets, stations, anomalies, colonies and territory.

### RTS
Real-time fleet command, movement, targeting, formations, ship abilities, orbital battles, carrier warfare and planetary sieges.

### Turn-based tactics
Grid or instance-based combat for boarding, invasions, base assaults, missions and competitive tactical encounters.

## Power model

```text
TotalPower = CharacterPower + FleetPower + EmpirePower + TechnologyPower + EconomicPower
```

Power is a reporting metric rather than an absolute combat outcome. Context modifiers, player skill, counters, encounter mechanics and normalization remain authoritative.

## Scaling

PvE scaling can increase boss health by level difference, party size and difficulty. Damage scaling should remain slower than health scaling to preserve counterplay.

PvP applies normalization to damage, critical chance, critical damage, healing, cooldown reduction and crowd-control duration.

## Boss design

Bosses use phase triggers such as health thresholds, timers, deaths and environmental transitions. Each phase can add abilities, adds, hazards, damage-type changes and arena rules.

## Economy

The strategic layer includes credits, minerals, gas, energy, data, dark matter, antimatter, nanites, quantum cores and artifacts. The character layer supports currencies, crafting, auction and trade.

## Persistence

Persistent state should include account, character, equipment, inventory, quests, guild membership, faction reputation, empire, planets, fleets, technology, diplomacy, territory, economic state and audit events.

## Authoritative execution

Clients submit intents. The server validates ownership, cooldowns, resources, permissions, range, state and timing before mutating authoritative state.
