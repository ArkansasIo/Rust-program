use serde::{Deserialize, Serialize};
use crate::items::Item;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory { pub capacity:usize, pub items:Vec<Item> }
impl Inventory { pub fn new(capacity:usize)->Self{Self{capacity,items:Vec::new()}} pub fn add(&mut self,item:Item)->bool{if self.items.len()>=self.capacity{return false} self.items.push(item);true} pub fn remove(&mut self,id:&str)->Option<Item>{self.items.iter().position(|x|x.id==id).map(|i|self.items.remove(i))} }

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Equipment { pub weapon:Option<Item>, pub armor:Option<Item>, pub accessory:Option<Item> }
