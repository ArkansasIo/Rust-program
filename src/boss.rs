use serde::{Deserialize, Serialize};
use crate::enemies::BossType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BossPhase { pub id: u32, pub hp_threshold: Option<f32>, pub time_limit_seconds: Option<u64>, pub abilities: Vec<String>, pub spawn_adds: Vec<String>, pub arena_modifier: Option<String> }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BossDefinition { pub id: String, pub name: String, pub boss_type: BossType, pub base_hp: f32, pub base_damage: f32, pub phases: Vec<BossPhase>, pub immunities: Vec<String>, pub enrage_seconds: Option<u64>, pub loot_table: String }
#[derive(Clone, Debug, Default)]
pub struct BossState { pub phase: usize, pub hp_percent: f32, pub elapsed_seconds: u64, pub enraged: bool }
impl BossDefinition {
    pub fn phase_for(&self, hp_percent: f32) -> usize { self.phases.iter().enumerate().filter(|(_, p)| p.hp_threshold.map(|t| hp_percent <= t).unwrap_or(false)).map(|(i, _)| i).max().unwrap_or(0) }
}
