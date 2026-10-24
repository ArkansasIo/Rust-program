use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)] pub struct Wallet { pub gold:u64, pub premium:u64 }
impl Wallet { pub fn deposit(&mut self,amount:u64){self.gold=self.gold.saturating_add(amount)} pub fn withdraw(&mut self,amount:u64)->bool{if self.gold<amount{false}else{self.gold-=amount;true}} }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct Recipe { pub id:String, pub ingredients:HashMap<String,u32>, pub output:String, pub output_count:u32, pub craft_time:u64 }
#[derive(Debug, Default)] pub struct CraftingSystem { pub recipes:HashMap<String,Recipe> }
impl CraftingSystem { pub fn can_craft(&self,recipe:&Recipe,items:&HashMap<String,u32>)->bool{recipe.ingredients.iter().all(|(id,n)|items.get(id).copied().unwrap_or(0)>=*n)} pub fn craft(&self,recipe_id:&str,items:&mut HashMap<String,u32>)->Option<String>{let r=self.recipes.get(recipe_id)?;if !self.can_craft(r,items){return None}for (id,n) in &r.ingredients{*items.get_mut(id).unwrap()-=*n;}Some(r.output.clone())} }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct AuctionListing { pub id:u64, pub seller:u64, pub item_id:String, pub quantity:u32, pub price:u64 }
#[derive(Debug, Default)] pub struct AuctionHouse { pub listings:HashMap<u64,AuctionListing>, pub next_id:u64 }
impl AuctionHouse { pub fn list(&mut self,seller:u64,item_id:String,quantity:u32,price:u64)->u64{self.next_id+=1;let id=self.next_id;self.listings.insert(id,AuctionListing{id,seller,item_id,quantity,price});id} pub fn buy(&mut self,id:u64)->Option<AuctionListing>{self.listings.remove(&id)} }
