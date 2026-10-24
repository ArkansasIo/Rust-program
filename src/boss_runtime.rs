use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossPhase {
    pub phase: u32,
    pub health_threshold: f32,
    pub damage_multiplier: f32,
    pub adds: u32,
    pub mechanic: String,
    pub arena_modifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossState {
    pub max_hp: f32,
    pub hp: f32,
    pub phase_index: usize,
    pub phases: Vec<BossPhase>,
}

impl BossState {
    pub fn new(max_hp: f32, phases: Vec<BossPhase>) -> Self {
        Self { max_hp: max_hp.max(1.0), hp: max_hp.max(1.0), phase_index: 0, phases }
    }

    pub fn health_percent(&self) -> f32 { (self.hp / self.max_hp).clamp(0.0, 1.0) }

    pub fn update_phase(&mut self) -> Option<&BossPhase> {
        let percent = self.health_percent();
        let mut selected = 0usize;
        for (index, phase) in self.phases.iter().enumerate() {
            if percent <= phase.health_threshold { selected = index; }
        }
        if selected != self.phase_index {
            self.phase_index = selected;
            self.phases.get(self.phase_index)
        } else { None }
    }

    pub fn apply_damage(&mut self, amount: f32) -> f32 {
        let before = self.hp;
        self.hp = (self.hp - amount.max(0.0)).max(0.0);
        before - self.hp
    }

    pub fn defeated(&self) -> bool { self.hp <= 0.0 }
}
