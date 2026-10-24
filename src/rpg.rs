use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)] pub enum CharacterClass { Warrior, Knight, Rogue, Ranger, Mage, Cleric, Paladin, Necromancer, Monk, Bard }
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)] pub enum Rarity { Common, Uncommon, Rare, Epic, Legendary, Mythic }
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)] pub enum QuestState { Available, Active, Completed, Failed }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct Skill { pub id:String, pub name:String, pub level:u32, pub power:f32, pub cooldown:u64 }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct Talent { pub id:String, pub name:String, pub rank:u32, pub max_rank:u32 }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct Quest { pub id:String, pub name:String, pub state:QuestState, pub required_level:u32, pub xp_reward:u64, pub gold_reward:u64 }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct CharacterProgression { pub level:u32, pub xp:u64, pub class:CharacterClass, pub skills:HashMap<String,Skill>, pub talents:HashMap<String,Talent>, pub quests:HashMap<String,Quest>, pub achievements:HashSet<String> }
impl Default for CharacterProgression { fn default()->Self{Self{level:1,xp:0,class:CharacterClass::Warrior,skills:HashMap::new(),talents:HashMap::new(),quests:HashMap::new(),achievements:HashSet::new()}} }
impl CharacterProgression { pub fn xp_to_next(&self)->u64{500*self.level as u64*self.level as u64} pub fn add_xp(&mut self, mut xp:u64){while xp>0{let need=self.xp_to_next().saturating_sub(self.xp);if xp<need{self.xp+=xp;break}xp-=need;self.xp=0;self.level+=1;}} pub fn learn_skill(&mut self,skill:Skill){self.skills.insert(skill.id.clone(),skill);} pub fn rank_talent(&mut self,id:&str)->bool{if let Some(t)=self.talents.get_mut(id){if t.rank<t.max_rank{t.rank+=1;return true}}false} pub fn complete_quest(&mut self,id:&str)->Option<(u64,u64)>{let q=self.quests.get_mut(id)?;if q.state==QuestState::Active{q.state=QuestState::Completed;Some((q.xp_reward,q.gold_reward))}else{None}} pub fn unlock_achievement(&mut self,id:impl Into<String>){self.achievements.insert(id.into());} }
