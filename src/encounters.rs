use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EncounterDifficulty { Story, Normal, Heroic, Mythic, Nightmare, World }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncounterDefinition { pub id: String, pub name: String, pub difficulty: EncounterDifficulty, pub min_players: u32, pub max_players: u32, pub enemy_ids: Vec<String>, pub objective_ids: Vec<String>, pub time_limit_seconds: Option<u64>, pub difficulty_factor: f32 }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EncounterState { pub active: bool, pub completed: bool, pub elapsed_seconds: u64, pub defeated_enemies: u32, pub objectives_completed: u32 }
impl EncounterState { pub fn complete(&mut self) { self.active = false; self.completed = true; } }
