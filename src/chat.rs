use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub enum ChatChannel { Say, Zone, Party, Guild, Whisper, System }
#[derive(Debug, Clone)] pub struct ChatMessage { pub sender_id:u64, pub sender_name:String, pub channel:ChatChannel, pub text:String, pub tick:u64 }
#[derive(Debug, Default)] pub struct ChatService { history: Vec<ChatMessage>, muted: HashMap<u64,bool> }
impl ChatService {
 pub fn send(&mut self,msg:ChatMessage)->bool { if self.muted.get(&msg.sender_id).copied().unwrap_or(false) {return false;} self.history.push(msg); true }
 pub fn mute(&mut self,id:u64){self.muted.insert(id,true);} pub fn unmute(&mut self,id:u64){self.muted.remove(&id);}
 pub fn recent(&self,limit:usize)->Vec<ChatMessage>{self.history.iter().rev().take(limit).cloned().collect()}
}
