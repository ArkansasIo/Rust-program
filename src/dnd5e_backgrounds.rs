//! Original background system for character creation.
use serde::{Deserialize, Serialize};
use crate::dnd5e_skills::Skill;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background { pub id:String, pub name:String, pub description:String, pub skill_proficiencies:Vec<Skill>, pub tool_proficiencies:Vec<String>, pub languages:Vec<String>, pub starting_gold:u64, pub feature_id:String }
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundRegistry { pub entries:Vec<Background> }
impl BackgroundRegistry { pub fn find(&self,id:&str)->Option<&Background>{self.entries.iter().find(|b|b.id==id)} }
