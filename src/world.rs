use crate::core::{EntityId, GameTick, Vec2};
use crate::entities::Combatant;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct World { pub tick:GameTick, pub entities:HashMap<EntityId,Combatant> }
impl World { pub fn spawn(&mut self,entity:Combatant){self.entities.insert(entity.id,entity);} pub fn get(&self,id:EntityId)->Option<&Combatant>{self.entities.get(&id)} pub fn get_mut(&mut self,id:EntityId)->Option<&mut Combatant>{self.entities.get_mut(&id)} pub fn nearby(&self,pos:Vec2,radius:f32)->Vec<EntityId>{self.entities.values().filter(|e|e.position.distance(pos)<=radius).map(|e|e.id).collect()} pub fn tick(&mut self){self.tick.0+=1;} }
