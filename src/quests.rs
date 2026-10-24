use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize)] pub enum Objective { Kill{enemy:String,count:u32,current:u32}, Collect{item:String,count:u32,current:u32}, Explore{zone:String}, Deliver{item:String,target:u64} }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct QuestDefinition { pub id:String, pub name:String, pub objectives:Vec<Objective>, pub xp:u64, pub gold:u64 }
impl QuestDefinition { pub fn complete(&self)->bool{self.objectives.iter().all(|o|match o{Objective::Kill{count,current,..}|Objective::Collect{count,current,..}=>current>=count,Objective::Explore{..}|Objective::Deliver{..}=>true})} }
#[derive(Debug,Default)] pub struct QuestLog { pub active:HashMap<String,QuestDefinition>, pub completed:Vec<String> }
impl QuestLog { pub fn accept(&mut self,q:QuestDefinition){self.active.insert(q.id.clone(),q)} pub fn turn_in(&mut self,id:&str)->Option<(u64,u64)>{let q=self.active.get(id)?;if !q.complete(){return None}let q=self.active.remove(id).unwrap();self.completed.push(q.id);Some((q.xp,q.gold))} }
