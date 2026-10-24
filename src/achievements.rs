use serde::{Deserialize,Serialize};
use std::collections::HashSet;
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct Achievement {pub id:String,pub name:String,pub description:String,pub points:u32}
#[derive(Debug,Default)] pub struct AchievementService { unlocked:std::collections::HashMap<u64,HashSet<String>> }
impl AchievementService { pub fn unlock(&mut self,player:u64,id:impl Into<String>)->bool{self.unlocked.entry(player).or_default().insert(id.into())} pub fn has(&self,player:u64,id:&str)->bool{self.unlocked.get(&player).is_some_and(|s|s.contains(id))} pub fn list(&self,player:u64)->Vec<String>{self.unlocked.get(&player).map(|s|s.iter().cloned().collect()).unwrap_or_default()} }
