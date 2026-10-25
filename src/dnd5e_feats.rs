//! Feats and talent hooks used by character progression.
use serde::{Deserialize, Serialize};
use crate::dnd5e::{AbilityScore, AbilityScores};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feat { pub id:String, pub name:String, pub description:String, pub ability_increase:Option<(AbilityScore,i32)>, pub prerequisite_level:u32, pub tags:Vec<String> }
impl Feat { pub fn eligible(&self,level:u32)->bool{level>=self.prerequisite_level} pub fn apply(&self,scores:&mut AbilityScores){if let Some((a,v))=self.ability_increase{match a{AbilityScore::Strength=>scores.strength+=v,AbilityScore::Dexterity=>scores.dexterity+=v,AbilityScore::Constitution=>scores.constitution+=v,AbilityScore::Intelligence=>scores.intelligence+=v,AbilityScore::Wisdom=>scores.wisdom+=v,AbilityScore::Charisma=>scores.charisma+=v}}} }
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeatBook { pub feats:Vec<Feat> }
impl FeatBook { pub fn find(&self,id:&str)->Option<&Feat>{self.feats.iter().find(|f|f.id==id)} }
