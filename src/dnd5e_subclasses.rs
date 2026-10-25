//! Original subclass hooks for the twelve core class archetypes.
use serde::{Deserialize,Serialize};
use crate::dnd5e::CharacterClass;
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SubclassDefinition { pub id:String,pub name:String,pub class:CharacterClass, pub unlock_level:u32,pub features:Vec<String>,pub role:String }
pub fn starter_subclasses()->Vec<SubclassDefinition>{use CharacterClass::*;vec![SubclassDefinition{id:"berserker_path".into(),name:"Path of the Berserker".into(),class:Barbarian,unlock_level:3,features:vec!["frenzy".into(),"rage_mastery".into()],role:"frontline".into()},SubclassDefinition{id:"battle_mastery".into(),name:"Battle Mastery".into(),class:Fighter,unlock_level:3,features:vec!["maneuvers".into(),"combat_discipline".into()],role:"tactical".into()},SubclassDefinition{id:"arcane_school".into(),name:"Arcane School".into(),class:Wizard,unlock_level:2,features:vec!["school_specialization".into()],role:"caster".into()},SubclassDefinition{id:"shadow_path".into(),name:"Shadow Path".into(),class:Monk,unlock_level:3,features:vec!["shadow_step".into()],role:"skirmisher".into()}]}
