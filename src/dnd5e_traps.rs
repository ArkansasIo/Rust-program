//! Traps, hazards, and environmental challenges.
use serde::{Deserialize,Serialize};
use crate::dnd5e::DamageType;
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct TrapDefinition{pub id:String,pub name:String,pub detection_dc:i32,pub disarm_dc:i32,pub save_dc:i32,pub damage_dice:(u8,u8),pub damage_type:DamageType,pub trigger:String,pub reset:bool}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct HazardDefinition{pub id:String,pub name:String,pub save_dc:i32,pub damage_dice:(u8,u8),pub damage_type:DamageType,pub duration_rounds:u32,pub area_meters:f32}
