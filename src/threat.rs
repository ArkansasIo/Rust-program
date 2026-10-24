use crate::core::EntityId;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)] pub struct ThreatTable { pub threat:HashMap<EntityId,f32> }
impl ThreatTable { pub fn add(&mut self,id:EntityId,value:f32){*self.threat.entry(id).or_default()+=value.max(0.0)} pub fn reduce(&mut self,id:EntityId,value:f32){if let Some(v)=self.threat.get_mut(&id){*v=(*v-value.max(0.0)).max(0.0)}} pub fn highest(&self)->Option<EntityId>{self.threat.iter().max_by(|a,b|a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)).map(|(id,_)|*id)} pub fn clear(&mut self){self.threat.clear()} }
