# MMORPG Backend Specification

## Services

| Service | Responsibility |
|---|---|
| Auth | Accounts, roles, sessions, expiration |
| Character | Character identity, progression and position |
| Realm/Shard | Player placement and capacity |
| World | Zones, spawns, NPCs and simulation |
| Instance | Dungeon/raid lifecycle |
| Combat | Server-authoritative combat resolution |
| Replication | Per-player state deltas |
| Social | Parties, guilds, friends and chat |
| Economy | Currency, crafting, auction and trade |
| Persistence | Repository abstraction and snapshots |
| Security | Sequence validation, rate limits and anti-cheat hooks |
| PvP | Duels, arenas, battlegrounds and ranked matches |

## Login flow
```text
Authenticate account
 -> create session
 -> select character
 -> allocate shard
 -> load character snapshot
 -> enter world
 -> begin replication
```

## Save flow
```text
Gameplay event
 -> authoritative state mutation
 -> periodic snapshot
 -> repository save
 -> acknowledgement / telemetry
```

## Production database targets
Recommended tables include `accounts`, `sessions`, `characters`, `character_stats`, `inventory`, `equipment`, `quests`, `skills`, `talents`, `guilds`, `guild_members`, `friends`, `mail`, `wallets`, `auction_listings`, `trades`, `instances`, `pvp_matches`, `world_state`, and `audit_events`.

The repository currently exposes storage interfaces rather than binding gameplay to a specific SQL driver. This keeps local simulation and dedicated-server testing lightweight.
