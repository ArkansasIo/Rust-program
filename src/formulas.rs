pub fn soft_cap(raw: f32, cap: f32) -> f32 { if raw <= 0.0 { 0.0 } else { raw / (raw + cap) * cap } }
pub fn diminishing_returns(raw: f32, cap: f32, k: f32) -> f32 { if raw <= 0.0 { 0.0 } else { cap * (1.0 - (-raw / k.max(0.001)).exp()) } }
pub fn max_hp(vit: f32, con: f32, gear_hp: f32) -> f32 { vit * 15.0 + con * 10.0 + gear_hp }
pub fn physical_damage(str_: f32, weapon_multiplier: f32) -> f32 { str_ * 2.5 * weapon_multiplier }
pub fn crit_chance(dex: f32, gear_crit: f32) -> f32 { (dex * 0.04 + gear_crit).clamp(0.0, 100.0) }
pub fn armor_mitigation(armor: f32, attacker_level: f32, k: f32) -> f32 { (armor / (armor + k.max(1.0) * attacker_level.max(1.0))).clamp(0.0, 0.70) }
pub fn final_damage(incoming: f32, mitigation: f32) -> f32 { (incoming * (1.0 - mitigation.clamp(0.0, 0.95))).max(0.0) }
pub fn hit_chance(accuracy: f32, evasion: f32) -> f32 { if accuracy + evasion <= 0.0 { 1.0 } else { (accuracy / (accuracy + evasion)).clamp(0.0, 1.0) } }
pub fn boss_hp(base: f32, level_diff: f32, party_size: u32, difficulty: f32) -> f32 { let party = 1.0 + party_size.saturating_sub(1) as f32 * 0.65; base * (1.0 + level_diff.max(0.0) * 0.18) * party * difficulty.max(0.1) }
pub fn encounter_power_budget(player_power: f32, players: u32, difficulty: f32) -> f32 { player_power * players.max(1) as f32 * difficulty.max(0.1) }
pub fn fleet_power(ship_values: &[f32], crew_skill: f32, tech_multiplier: f32) -> f32 { ship_values.iter().sum::<f32>() * crew_skill.max(0.0) * tech_multiplier.max(0.0) }
pub fn research_time(cost: f32, science_output: f32, scientist_skill: f32) -> f32 { cost / (science_output.max(0.001) * scientist_skill.max(0.001)) }
pub fn trade_income(value: f32, security: f32, distance_modifier: f32) -> f32 { value.max(0.0) * security.clamp(0.0, 1.0) * distance_modifier.max(0.0) }
pub fn total_power(character: f32, fleet: f32, empire: f32, technology: f32, economy: f32) -> f32 { character + fleet + empire + technology + economy }
