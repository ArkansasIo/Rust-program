use crate::core::EntityId;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)] pub struct Party { pub id:u64, pub leader:EntityId, pub members:Vec<EntityId>, pub max_size:usize }
impl Party { pub fn add(&mut self,id:EntityId)->bool{if self.members.len()>=self.max_size||self.members.contains(&id){false}else{self.members.push(id);true}} pub fn remove(&mut self,id:EntityId){self.members.retain(|x|*x!=id)} pub fn contains(&self,id:EntityId)->bool{self.members.contains(&id)} }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct Guild { pub id:u64, pub name:String, pub leader:EntityId, pub members:Vec<EntityId>, pub level:u32, pub bank_gold:u64 }
#[derive(Debug, Default)] pub struct SocialSystem { pub parties:HashMap<u64,Party>, pub guilds:HashMap<u64,Guild>, pub next_id:u64 }
impl SocialSystem { pub fn create_party(&mut self,leader:EntityId,max_size:usize)->u64{self.next_id+=1;let id=self.next_id;self.parties.insert(id,Party{id,leader,members:vec![leader],max_size});id} pub fn create_guild(&mut self,name:String,leader:EntityId)->u64{self.next_id+=1;let id=self.next_id;self.guilds.insert(id,Guild{id,name,leader,members:vec![leader],level:1,bank_gold:0});id} }
