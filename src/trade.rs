use crate::items::Item;
use std::collections::HashMap;
#[derive(Debug,Clone)] pub struct TradeOffer { pub from:u64,pub to:u64,pub items:Vec<Item>,pub currency:u64 }
#[derive(Debug,Default)] pub struct TradeService { pub pending:HashMap<u64,TradeOffer>, next_id:u64 }
impl TradeService { pub fn create(&mut self,offer:TradeOffer)->u64{self.next_id+=1;let id=self.next_id;self.pending.insert(id,offer);id} pub fn accept(&mut self,id:u64)->Option<TradeOffer>{self.pending.remove(&id)} pub fn cancel(&mut self,id:u64)->Option<TradeOffer>{self.pending.remove(&id)} }
