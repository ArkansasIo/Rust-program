use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)] pub enum TacticalAction { Move, Attack, Ability, Overwatch, UseItem, Defend, Interact }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct TacticalUnit { pub id: u64, pub name: String, pub initiative: f32, pub speed: f32, pub tactical_skill: f32, pub equipment_bonus: f32, pub action_points: u32 }
#[derive(Clone, Debug, Default, Serialize, Deserialize)] pub struct TurnState { pub round: u32, pub active_index: usize, pub order: Vec<u64> }
impl TacticalUnit { pub fn initiative(&self) -> f32 { self.speed + self.tactical_skill + self.equipment_bonus } }
impl TurnState { pub fn build_order(units: &[TacticalUnit]) -> Vec<u64> { let mut v=units.to_vec(); v.sort_by(|a,b| b.initiative().partial_cmp(&a.initiative()).unwrap_or(std::cmp::Ordering::Equal)); v.into_iter().map(|u|u.id).collect() } }
