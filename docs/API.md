# Rust Battle Engine API Surface

## Core
- `EntityId`
- `TeamId`
- `GameTick`
- `Vec2`

## Simulation
- `CombatSystem`
- `Ability`
- `Cooldowns`
- `StatusController`
- `ThreatTable`
- `AiController`
- `World`
- `GameEngine`

## RPG
- `Character`
- `CharacterProgression`
- `CharacterClass`
- `QuestLog`
- `AchievementService`
- `Inventory`
- `Equipment`
- `CraftingSystem`
- `AuctionHouse`

## MMORPG
- `AuthService`
- `SessionManager`
- `ShardManager`
- `InstanceManager`
- `ReplicationState`
- `ChatService`
- `FriendService`
- `PvPService`
- `TradeService`
- `Repository`
- `MemoryRepository`

## Protocol
`ClientMessage`, `ServerMessage`, `Packet`, and `SequenceGuard` form a transport-neutral command/event contract.
