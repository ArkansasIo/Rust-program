use serde::{Deserialize, Serialize};
use crate::core::Vec2;
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct Zone { pub id:String,pub name:String, pub level_min:u32,pub level_max:u32,pub position:Vec2,pub radius:f32 }
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct SpawnPoint { pub id:u64,pub zone:String,pub enemy_id:String,pub count:u32,pub respawn_ticks:u64 }
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct Dungeon { pub id:String,pub name:String,pub required_level:u32,pub party_size:u32,pub boss_ids:Vec<String>,pub loot_table:String }
#[derive(Debug,Clone,Serialize,Deserialize)] pub struct Raid { pub id:String,pub name:String,pub required_level:u32,pub max_players:u32,pub boss_ids:Vec<String>,pub loot_table:String }
#[derive(Debug,Default)] pub struct WorldContent { pub zones:Vec<Zone>,pub spawns:Vec<SpawnPoint>,pub dungeons:Vec<Dungeon>,pub raids:Vec<Raid> }
