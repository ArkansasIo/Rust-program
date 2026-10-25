//! Character builder and level progression facade.
use serde::{Deserialize, Serialize};
use crate::dnd5e::{AbilityScores, CharacterClass, CharacterSheet, Race};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterBuild { pub name:String, pub race:Race, pub class:CharacterClass, pub abilities:AbilityScores, pub background_id:Option<String>, pub feat_ids:Vec<String> }
impl CharacterBuild { pub fn create(self,id:u64)->CharacterSheet { let mut c=CharacterSheet::new(id,self.name,self.class,self.race);c.abilities=self.abilities;c.max_hp=crate::dnd5e::hit_points(c.level,c.hit_die,c.ability_modifier(crate::dnd5e::AbilityScore::Constitution),None);c.current_hp=c.max_hp;c.armor_class=10+c.ability_modifier(crate::dnd5e::AbilityScore::Dexterity);c } }
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct LevelReward { pub level:u32,pub feature_ids:Vec<String>,pub spell_slots:[u8;10],pub feat_choice:bool }
pub fn xp_for_level(level:u32)->u64{let l=level.clamp(1,20);if l==1{0}else{(l as u64-1)*(l as u64-1)*1000}}
pub fn level_for_xp(xp:u64)->u32{(1..=20).rev().find(|&l|xp>=xp_for_level(l)).unwrap_or(1)}
