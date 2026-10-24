# Combat and Balance Specification

## Attributes

STR, DEX, CON, INT, WIS, VIT, SPI, LCK, WIL and CHA form the primary attribute layer.

## Sub-attributes

Physical Power, Armor Penetration, Attack Speed, Accuracy, Evasion, Weapon Handling, Critical Damage, Spell Power, Elemental Mastery, Cast Speed, Magic Penetration, Mana Efficiency, Overcharge, HP Regen, Shield Strength, Damage Reduction, Block and Parry.

## Resources

HP, MP, Stamina, Energy, Rage, Focus, Faith, Aether, Heat and Sanity.

## Damage model

```text
FinalDamage = IncomingDamage × (1 - Mitigation)
Mitigation = Armor / (Armor + K × AttackerLevel)
```

Resistance layers can add flat reduction, percentage reduction, absorption, vulnerability and immunity.

## Critical model

Critical chance is derived from DEX and gear. PvP should cap or normalize critical effects to reduce burst variance.

## Crowd control

Recommended diminishing-return sequence:

```text
1st application = 100%
2nd application = 50%
3rd application = 25%
4th application = temporary immunity
```

## PvE/PvP conversion

The supplied design calls for separate PvE and PvP coefficients. Damage, critical chance, critical damage, healing, cooldown reduction and crowd-control duration should be normalized in competitive modes.

## Hard limits

Recommended starting limits from the design:

- PvP critical chance: 40%
- PvE critical chance: 70%
- PvP cooldown reduction: 35%
- PvE cooldown reduction: 60%
- Damage reduction: 70%
- PvP healing bonus: 50%

These are balance defaults and should remain data-driven.

## Item power

```text
StatValuePU = RawStat × Weight × ScalingCurve
```

Example starting weights include STR/DEX/INT = 1.0, Critical Chance = 2.5, Attack Speed = 2.0, Armor = 0.75 and Resilience = 3.0.

## Encounter budget

```text
EncounterPowerBudget = Σ(PlayerPower) × DifficultyFactor
```

Boss mechanics should stay within the intended power budget while preserving telegraphs, counterplay and limited-frequency mechanics.
