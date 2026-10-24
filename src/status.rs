use serde::{Deserialize, Serialize};
use crate::core::EntityId;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)] pub enum StatusKind { Buff, Debuff, Stun, Silence, Root, Slow, Poison, Bleed, Burn, Shield, Regeneration }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct StatusEffect { pub id:String, pub kind:StatusKind, pub source:EntityId, pub duration:u64, pub magnitude:f32, pub stacks:u32, pub max_stacks:u32 }
#[derive(Debug, Default, Clone, Serialize, Deserialize)] pub struct StatusController { pub effects:Vec<StatusEffect> }
impl StatusController { pub fn apply(&mut self,mut e:StatusEffect){if let Some(old)=self.effects.iter_mut().find(|x|x.id==e.id){old.duration=old.duration.max(e.duration);old.magnitude=e.magnitude;old.stacks=(old.stacks+e.stacks).min(e.max_stacks.max(1));}else{e.stacks=e.stacks.max(1).min(e.max_stacks.max(1));self.effects.push(e)}} pub fn tick(&mut self)->Vec<StatusEffect>{let mut expired=Vec::new();for e in &mut self.effects{e.duration=e.duration.saturating_sub(1);}let mut i=0;while i<self.effects.len(){if self.effects[i].duration==0{expired.push(self.effects.remove(i));}else{i+=1}}expired} pub fn has(&self,kind:StatusKind)->bool{self.effects.iter().any(|e|e.kind==kind)} pub fn clear(&mut self){self.effects.clear()} }
