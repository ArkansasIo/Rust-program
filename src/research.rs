use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)] pub enum ResearchBranch { Physics, Engineering, Biotech, Psionics, Economics, Military, Computing }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct Technology { pub id: String, pub name: String, pub branch: ResearchBranch, pub cost: f64, pub prerequisites: Vec<String>, pub unlocks: Vec<String> }
#[derive(Clone, Debug, Default, Serialize, Deserialize)] pub struct ResearchState { pub completed: Vec<String>, pub active: Option<String>, pub progress: f64 }
impl ResearchState { pub fn complete(&mut self, id: impl Into<String>) { let id=id.into(); if !self.completed.contains(&id) { self.completed.push(id); } self.active=None; self.progress=0.0; } }
