use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnemyRank { Common, Veteran, Elite, Rare, Named, Boss }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AiArchetype { Brute, Assassin, Caster, Controller, Summoner, Support, Tank, Sniper }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BossType { Dungeon, World, Raid, Event, Story, Mythic, Secret }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyArchetype {
    pub id: String, pub name: String, pub rank: EnemyRank, pub ai: AiArchetype,
    pub health_multiplier: f32, pub damage_multiplier: f32, pub skills: Vec<String>,
    pub immunities: Vec<String>, pub loot_multiplier: f32,
}
impl EnemyArchetype {
    pub fn baseline(rank: EnemyRank) -> f32 { match rank { EnemyRank::Common => 0.7, EnemyRank::Veteran => 1.35, EnemyRank::Elite => 2.5, EnemyRank::Rare => 2.0, EnemyRank::Named => 3.0, EnemyRank::Boss => 10.0 } }
}
