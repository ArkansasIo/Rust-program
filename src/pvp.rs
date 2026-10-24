use crate::core::{EntityId,TeamId};
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum MatchType { Duel,Arena,Battleground,Ranked }
#[derive(Debug,Clone)] pub struct PvPMatch { pub id:u64,pub kind:MatchType,pub players:Vec<EntityId>,pub teams:Vec<TeamId>,pub started:bool,pub finished:bool }
#[derive(Debug,Default)] pub struct PvPService { pub matches:Vec<PvPMatch>, next_id:u64 }
impl PvPService { pub fn create(&mut self,kind:MatchType,players:Vec<EntityId>,teams:Vec<TeamId>)->u64{self.next_id+=1;let id=self.next_id;self.matches.push(PvPMatch{id,kind,players,teams,started:false,finished:false});id} pub fn start(&mut self,id:u64)->bool{if let Some(m)=self.matches.iter_mut().find(|m|m.id==id){m.started=true;true}else{false}} pub fn finish(&mut self,id:u64)->bool{if let Some(m)=self.matches.iter_mut().find(|m|m.id==id){m.finished=true;true}else{false}} }
