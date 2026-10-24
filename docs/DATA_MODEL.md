# Data Model

## Account
`id`, `username`, `roles`, `disabled`

## Character
`id`, `account_id`, `name`, `class`, `level`, `attributes`, `resources`, `progression`, `zone_id`, `position`, `online`

## World
`Zone`, `SpawnPoint`, `Dungeon`, `Raid`, `Npc`, `DialogueTree`

## Runtime
`PlayerSession`, `Shard`, `Instance`, `ReplicationState`, `PvPMatch`

## Economy
`Wallet`, `Recipe`, `AuctionListing`, `TradeOffer`, `Item`

## Serialization
Serde is used for transport-safe and persistence-friendly structures. Avoid serializing internal caches unless they are explicitly part of the durable game state.
