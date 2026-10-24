use serde::{Deserialize, Serialize};
use crate::ships::Ship;
#[derive(Clone, Debug, Default, Serialize, Deserialize)] pub struct Fleet { pub id: String, pub name: String, pub flagship_id: Option<String>, pub ship_ids: Vec<String>, pub morale: f32, pub command_capacity: u32, pub sensor_range: f32 }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct FleetSnapshot { pub fleet: Fleet, pub ships: Vec<Ship> }
impl FleetSnapshot { pub fn power(&self, tech_multiplier: f32) -> f32 { self.ships.iter().map(|s| s.combat_value() * s.crew_skill.max(0.1)).sum::<f32>() * tech_multiplier.max(0.1) } }
