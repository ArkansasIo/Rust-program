//! Encounter budgets and encounter composition for dungeons and open world.
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncounterDifficulty { Trivial, Easy, Moderate, Hard, Deadly, Mythic }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterSlot { pub monster_id:String, pub count:u32, pub elite:bool, pub boss:bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterDefinition { pub id:String, pub name:String, pub difficulty:EncounterDifficulty, pub recommended_party:u32, pub level_min:u32, pub level_max:u32, pub slots:Vec<EncounterSlot>, pub arena:String, pub rewards:Vec<String> }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncounterBudget { pub budget:i32, pub used:i32 }
impl EncounterBudget { pub fn new(budget:i32)->Self{Self{budget,used:0}} pub fn can_add(&self,cost:i32)->bool{self.used+cost<=self.budget} pub fn add(&mut self,cost:i32)->bool{if self.can_add(cost){self.used+=cost;true}else{false}} pub fn remaining(&self)->i32{self.budget-self.used} }
