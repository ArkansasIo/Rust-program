# Sci-Fi MMORPG + 4X + RTS + Turn-Based Specification

## Universe hierarchy

```text
Galaxy
└── Sector
    └── Star System
        ├── Planet
        ├── Asteroid Field
        ├── Station
        └── Anomaly
```

## Factions

Factions influence research, ships, diplomacy, economy and special abilities. Example archetypes include federation, coalition, cybernetic collective, alien hive, corporate syndicate and rogue AI.

## Empire

Industry controls production. Science controls research. Economy controls credits. Influence controls diplomacy. Logistics controls fleet supply. Intelligence controls espionage. Stability controls internal resistance.

## Planet

Planets contain population, habitability, resource output, defense, stability and buildings such as mining complexes, research labs, shipyards, trade hubs, defense grids, orbital cannons and habitat domes.

## Fleet

Fleet classes include fighters, corvettes, frigates, destroyers, cruisers, battleships, carriers, dreadnoughts, titans and flagships.

## Components

Ships are modular. Weapons include lasers, plasma, railguns, missiles, ion cannons and particle beams. Defensive components include armor, shields and point defense. Systems include reactor, sensors, engines, AI core and targeting computer.

## Space combat

```text
HitChance = Accuracy / (Accuracy + TargetEvasion)
Damage = WeaponDamage × TechMultiplier × CommanderBonus
FinalDamage = Damage × (1 - ShieldReduction) × (1 - ArmorReduction)
```

## Research

Research branches include Physics, Engineering, Biotech, Psionics, Economics, Military and Computing. Technology nodes can unlock ships, weapons, buildings, travel systems and empire modifiers.

## Diplomacy

Relations include Neutral, Trade, Non-Aggression, Research Pact, Alliance, Vassal and War.

## Trade

```text
TradeIncome = TradeValue × RouteSecurity × DistanceModifier
```

Pirates, blockades and war zones can modify route security.

## Strategic AI

AI evaluates threat level, resource need, expansion opportunity and diplomatic risk. Supported strategic styles are Expansionist, Defensive, Economic, Aggressive, Technological and Diplomatic.

## MMO events

Persistent events can include alien invasions, supernovas, rogue-AI uprisings, artifact discoveries, faction wars, economic crises and cosmic anomalies.

## Victory conditions

Military, Economic, Technological, Diplomatic and Ascension victories are supported as strategic objectives.

## Server topology

The recommended topology is a galaxy service containing sector/world servers and system instances. Real-time RTS battles and turn-based ground instances can run as authoritative sub-services while empire state remains persistent.
