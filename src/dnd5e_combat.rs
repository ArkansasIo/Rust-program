//! Server-authoritative D20 attack, damage, and saving throw resolution.
use serde::{Deserialize, Serialize};
use crate::dnd5e::{AbilityScore, DamageType, ProficiencyTier, AbilityScores, resolve_d20};
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]
pub enum AttackAbility { Strength, Dexterity, Spellcasting(AbilityScore) }
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]
pub struct AttackProfile { pub ability:AttackAbility, pub proficient:bool, pub flat_bonus:i32, pub damage_dice:(u8,u8), pub damage_bonus:i32, pub damage_type:DamageType }
pub fn attack_bonus(scores:&AbilityScores,profile:AttackProfile,level:u32)->i32{let a=match profile.ability{AttackAbility::Strength=>AbilityScore::Strength,AttackAbility::Dexterity=>AbilityScore::Dexterity,AttackAbility::Spellcasting(a)=>a};scores.modifier_for(a)+if profile.proficient{crate::dnd5e::proficiency_bonus(level)}else{0}+profile.flat_bonus}
pub fn resolve_attack(scores:&AbilityScores,profile:AttackProfile,level:u32,d20:u8,ac:i32)->crate::dnd5e::CheckResult{resolve_d20(d20,attack_bonus(scores,profile,level),ac)}
pub fn saving_throw(scores:&AbilityScores,ability:AbilityScore,proficient:bool,level:u32,d20:u8,dc:i32)->crate::dnd5e::CheckResult{let m=scores.modifier_for(ability)+if proficient{crate::dnd5e::proficiency_bonus(level)}else{0};resolve_d20(d20,m,dc)}
pub fn apply_damage(raw:i32,resistant:bool,immune:bool,vulnerable:bool)->i32{if immune{0}else{let mut n=raw.max(0);if resistant{n/=2}if vulnerable{n*=2}n}}
