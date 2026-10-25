//! D20-compatible reward tables layered over the engine's existing loot system.
use serde::{Deserialize,Serialize};
#[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]
pub enum RewardRarity{Common,Uncommon,Rare,Epic,Legendary,Mythic}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct RewardEntry{pub item_id:String,pub rarity:RewardRarity,pub weight:u32,pub min_level:u32,pub max_level:u32}
#[derive(Debug,Clone,Default,Serialize,Deserialize)]
pub struct RewardTable{pub id:String,pub entries:Vec<RewardEntry>}
impl RewardTable{pub fn eligible(&self,level:u32)->Vec<&RewardEntry>{self.entries.iter().filter(|e|level>=e.min_level&&level<=e.max_level).collect()}pub fn total_weight(&self,level:u32)->u64{self.eligible(level).iter().map(|e|e.weight as u64).sum()}}
