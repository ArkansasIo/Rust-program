//! Equipment definitions and derived defense calculations.
use serde::{Deserialize, Serialize};
use crate::dnd5e::{AbilityScore, AbilityScores, DamageType};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquipmentSlot { Head, Shoulders, Chest, Hands, Legs, Feet, MainHand, OffHand, Ranged, Ring1, Ring2, Amulet, Relic }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmorClass { Light, Medium, Heavy, Shield }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentDefinition { pub id:String, pub name:String, pub slot:EquipmentSlot, pub armor_class:Option<ArmorClass>, pub armor_bonus:i32, pub damage:(u8,u8), pub ability_modifiers:[(AbilityScore,i32);2], pub resistances:Vec<DamageType>, pub value:u64 }
impl EquipmentDefinition { pub fn apply(&self,scores:&mut AbilityScores){ for (a,v) in self.ability_modifiers { match a { AbilityScore::Strength=>scores.strength+=v, AbilityScore::Dexterity=>scores.dexterity+=v, AbilityScore::Constitution=>scores.constitution+=v, AbilityScore::Intelligence=>scores.intelligence+=v, AbilityScore::Wisdom=>scores.wisdom+=v, AbilityScore::Charisma=>scores.charisma+=v } } } }
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EquipmentSet { pub items:Vec<EquipmentDefinition> }
impl EquipmentSet { pub fn armor_bonus(&self)->i32{self.items.iter().map(|x|x.armor_bonus).sum()} pub fn resistances(&self)->Vec<DamageType>{let mut out=Vec::new();for i in &self.items{for r in &i.resistances{if !out.contains(r){out.push(*r)}}}out} }
