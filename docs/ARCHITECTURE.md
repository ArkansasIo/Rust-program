# Rust Battle Engine Architecture

## Scope
A modular, server-authoritative Rust foundation for RPG and MMORPG games. The repository separates deterministic gameplay simulation from persistence, networking, social services, and world orchestration.

## Runtime layers
1. **Core** — entity IDs, ticks, vectors, teams, shared primitives.
2. **Simulation** — combat, abilities, stats, resources, status effects, threat, physics, AI.
3. **RPG** — classes, progression, skills, talents, quests, achievements, inventory, equipment, items, loot, crafting and economy.
4. **World** — zones, spawns, dungeons, raids, NPCs, dialogue and instances.
5. **MMORPG services** — accounts, sessions, characters, shards, replication, chat, friends, parties, guilds, mail, matchmaking, trading and PvP.
6. **Infrastructure** — persistence traits, JSON serialization, protocol messages, rate limiting and anti-cheat hooks.

## Authoritative simulation
Clients send intents rather than authoritative state. The server validates commands, advances game ticks, executes simulation, emits events, and replicates approved state deltas.

```text
Client
  -> Network Packet
  -> Auth / Rate Limit / Sequence Validation
  -> Command Router
  -> World + Character + Combat Simulation
  -> Event Bus
  -> Persistence Snapshot
  -> Replication Delta
  -> Clients
```

## Expansion points
The current persistence layer is intentionally storage-agnostic. A production deployment can implement `Repository<K,V>` with MySQL/PostgreSQL/Redis-backed adapters without changing gameplay modules.

The networking types are transport-neutral. TCP, WebSocket, QUIC, or another reliable transport can be placed around the protocol layer.
