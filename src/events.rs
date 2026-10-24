use crate::combat::CombatEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent { Combat(CombatEvent), Spawn(u64), Death(u64), Victory(u32) }
#[derive(Debug, Default)]
pub struct EventBus { events:Vec<GameEvent> }
impl EventBus { pub fn emit(&mut self,event:GameEvent){self.events.push(event)} pub fn drain(&mut self)->Vec<GameEvent>{std::mem::take(&mut self.events)} }
