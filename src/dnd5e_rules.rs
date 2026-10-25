//! Shared rules calculations and server-side validation.
use serde::{Deserialize, Serialize};
use crate::dnd5e::{AbilityScore, AbilityScores, AdvantageState, CheckResult, ProficiencyTier, resolve_d20};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestKind { Short, Long }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckKind { Ability(AbilityScore), SavingThrow(AbilityScore), Skill(crate::dnd5e_skills::Skill), Attack }
pub fn resolve_check(kind:CheckKind,scores:&AbilityScores,proficiency:ProficiencyTier,level:u32,d20:u8,dc:i32)->CheckResult { let ability=match kind{CheckKind::Ability(a)|CheckKind::SavingThrow(a)=>a,CheckKind::Skill(s)=>s.ability(),CheckKind::Attack=>AbilityScore::Strength}; let bonus=scores.modifier_for(ability)+proficiency.bonus(level);resolve_d20(d20,bonus,dc) }
pub fn contested_check(a:u8,a_mod:i32,b:u8,b_mod:i32)->std::cmp::Ordering{(a as i32+a_mod).cmp(&(b as i32+b_mod))}
pub fn passive(modifier:i32,proficiency:ProficiencyTier,level:u32,advantage:AdvantageState)->i32{let base=10+modifier+proficiency.bonus(level);match advantage{AdvantageState::Normal=>base,AdvantageState::Advantage=>base+5,AdvantageState::Disadvantage=>base-5}}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct RestResult { pub restored_hp:i32,pub restored_slots:bool,pub removed_conditions:usize }
