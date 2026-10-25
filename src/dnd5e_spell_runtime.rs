//! Runtime spellcasting state, concentration, preparation, and known spells.
use serde::{Deserialize,Serialize};
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct SpellCasterState { pub prepared:Vec<String>, pub known:Vec<String>, pub slots:[u8;10], pub used:[u8;10], pub concentration:Option<String> }
impl Default for SpellCasterState{fn default()->Self{Self{prepared:Vec::new(),known:Vec::new(),slots:[0;10],used:[0;10],concentration:None}}}
impl SpellCasterState{pub fn can_cast(&self,spell_id:&str,level:u8)->bool{let i=level as usize;i<10&&self.slots[i]>self.used[i]&&(level==0||self.prepared.iter().any(|x|x==spell_id)||self.known.iter().any(|x|x==spell_id))}pub fn cast(&mut self,spell_id:&str,level:u8)->bool{if !self.can_cast(spell_id,level){return false}if level>0{self.used[level as usize]+=1}true}pub fn begin_concentration(&mut self,spell_id:impl Into<String>){self.concentration=Some(spell_id.into())}pub fn break_concentration(&mut self){self.concentration=None}pub fn recover_slots(&mut self){self.used=[0;10]}}
