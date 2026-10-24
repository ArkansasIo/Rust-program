use std::collections::HashMap;
use crate::core::GameTick;
use crate::time::Deadline;

#[derive(Debug, Clone, Copy)] pub struct Cooldown { pub duration: u64, pub ready_at: GameTick }
impl Cooldown { pub fn new(duration:u64)->Self{Self{duration,ready_at:GameTick(0)}} pub fn ready(&self,now:GameTick)->bool{now.0>=self.ready_at.0} pub fn trigger(&mut self,now:GameTick){self.ready_at=GameTick(now.0.saturating_add(self.duration));} pub fn remaining(&self,now:GameTick)->u64{self.ready_at.0.saturating_sub(now.0)} pub fn deadline(&self)->Deadline{Deadline(self.ready_at)} }
#[derive(Debug,Default)] pub struct CooldownBook{entries:HashMap<u32,Cooldown>}
impl CooldownBook { pub fn set(&mut self,id:u32,duration:u64){self.entries.insert(id,Cooldown::new(duration));} pub fn ready(&self,id:u32,now:GameTick)->bool{self.entries.get(&id).map(|c|c.ready(now)).unwrap_or(true)} pub fn trigger(&mut self,id:u32,now:GameTick){self.entries.entry(id).or_insert_with(||Cooldown::new(0)).trigger(now);} pub fn remaining(&self,id:u32,now:GameTick)->u64{self.entries.get(&id).map(|c|c.remaining(now)).unwrap_or(0)} }
