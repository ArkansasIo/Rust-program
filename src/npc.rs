use crate::core::EntityId;
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum NpcKind { Vendor,QuestGiver,Trainer,Enemy,Neutral,Guard }
#[derive(Debug,Clone)] pub struct Npc {pub id:EntityId,pub template_id:String,pub name:String,pub kind:NpcKind,pub level:u32,pub alive:bool}
#[derive(Debug,Default)] pub struct NpcRegistry {pub npcs:Vec<Npc>}
impl NpcRegistry {pub fn add(&mut self,npc:Npc){self.npcs.push(npc)} pub fn find(&self,id:EntityId)->Option<&Npc>{self.npcs.iter().find(|n|n.id==id)} pub fn by_kind(&self,kind:NpcKind)->Vec<&Npc>{self.npcs.iter().filter(|n|n.kind==kind).collect()} }
