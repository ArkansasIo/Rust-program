use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EffectKind { Buff, Debuff, CrowdControl, DamageOverTime, HealOverTime, Shield, Immunity, Vulnerability }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ControlKind { Stun, Freeze, Root, Silence, Fear, Charm, Sleep, Knockback, Knockdown }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Effect { pub id: String, pub kind: EffectKind, pub control: Option<ControlKind>, pub magnitude: f32, pub duration_seconds: f32, pub stacks: u32, pub source_id: Option<u64> }
#[derive(Clone, Debug, Default)]
pub struct EffectController { pub active: Vec<Effect> }
impl EffectController {
    pub fn apply(&mut self, effect: Effect) { self.active.push(effect); }
    pub fn tick(&mut self, dt: f32) { for e in &mut self.active { e.duration_seconds = (e.duration_seconds - dt).max(0.0); } self.active.retain(|e| e.duration_seconds > 0.0); }
    pub fn has_control(&self, control: ControlKind) -> bool { self.active.iter().any(|e| e.control == Some(control)) }
    pub fn clear_debuffs(&mut self) { self.active.retain(|e| !matches!(e.kind, EffectKind::Debuff | EffectKind::CrowdControl | EffectKind::DamageOverTime)); }
    pub fn diminishing_control_multiplier(applications: u32) -> f32 { match applications { 0 => 1.0, 1 => 0.5, 2 => 0.25, _ => 0.0 } }
}
