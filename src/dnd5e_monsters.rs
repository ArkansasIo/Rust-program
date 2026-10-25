//! Data-driven monster and NPC stat blocks.
use serde::{Deserialize, Serialize};
use crate::dnd5e::{AbilityScores, CreatureSize, DamageType};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonsterRole { Brute, Tank, Assassin, Caster, Controller, Support, Summoner, Skirmisher, Artillery, Boss }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterAction { pub id:String, pub name:String, pub attack_bonus:i32, pub damage_dice:(u8,u8), pub damage_bonus:i32, pub damage_type:DamageType, pub range:i32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDefinition { pub id:String, pub name:String, pub level:u32, pub role:MonsterRole, pub size:CreatureSize, pub armor_class:i32, pub max_hp:i32, pub speed:i32, pub abilities:AbilityScores, pub resistances:Vec<DamageType>, pub immunities:Vec<DamageType>, pub actions:Vec<MonsterAction>, pub loot_table:Option<String> }
impl MonsterDefinition { pub fn is_boss(&self)->bool { self.role==MonsterRole::Boss } pub fn effective_damage(&self,amount:i32,kind:DamageType)->i32 { if self.immunities.contains(&kind){0}else if self.resistances.contains(&kind){amount/2}else{amount} } }
