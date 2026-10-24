use std::collections::HashMap;
#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum SecurityDecision { Allow,Deny }
#[derive(Debug,Default)] pub struct RateLimiter { buckets:HashMap<u64,(u64,u64)> }
impl RateLimiter { pub fn allow(&mut self,id:u64,now:u64,limit:u64,window:u64)->SecurityDecision{let e=self.buckets.entry(id).or_insert((now,0));if now.saturating_sub(e.0)>=window{*e=(now,0);} if e.1>=limit{SecurityDecision::Deny}else{e.1+=1;SecurityDecision::Allow}} }
#[derive(Debug,Default)] pub struct AntiCheat { pub rejected_packets:u64,pub rejected_commands:u64 }
impl AntiCheat { pub fn reject_packet(&mut self){self.rejected_packets+=1;} pub fn reject_command(&mut self){self.rejected_commands+=1;} }
