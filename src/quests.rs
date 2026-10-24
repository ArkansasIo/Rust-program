use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Objective {
    Kill { enemy: String, count: u32, current: u32 },
    Collect { item: String, count: u32, current: u32 },
    Explore { zone: String },
    Deliver { item: String, target: u64 },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDefinition { pub id: String, pub name: String, pub objectives: Vec<Objective>, pub xp: u64, pub gold: u64 }
impl QuestDefinition {
    pub fn complete(&self) -> bool {
        self.objectives.iter().all(|objective| match objective {
            Objective::Kill { count, current, .. } | Objective::Collect { count, current, .. } => current >= count,
            // These objectives are complete only when progress is explicitly recorded.
            Objective::Explore { .. } | Objective::Deliver { .. } => false,
        })
    }
}
#[derive(Debug, Default)]
pub struct QuestLog { pub active: HashMap<String, QuestDefinition>, pub completed: Vec<String> }
impl QuestLog {
    pub fn accept(&mut self, quest: QuestDefinition) { self.active.insert(quest.id.clone(), quest); }
    pub fn turn_in(&mut self, id: &str) -> Option<(u64, u64)> {
        let quest = self.active.get(id)?;
        if !quest.complete() { return None; }
        let quest = self.active.remove(id)?;
        self.completed.push(quest.id);
        Some((quest.xp, quest.gold))
    }
}
