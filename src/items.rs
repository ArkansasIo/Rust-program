use serde::{Deserialize, Serialize};
use crate::combat::DamageType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemKind { Weapon{damage:f32,damage_type:DamageType}, Armor{armor:f32}, Consumable{heal:f32}, Quest }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item { pub id:String, pub name:String, pub level:u32, pub kind:ItemKind }
