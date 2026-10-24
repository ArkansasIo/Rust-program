use crate::core::EntityId;
use crate::rpg::{CharacterClass, CharacterProgression};
use crate::stats::{Attributes, Resources};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: EntityId,
    pub account_id: u64,
    pub name: String,
    pub class: CharacterClass,
    pub level: u32,
    pub attributes: Attributes,
    pub resources: Resources,
    pub progression: CharacterProgression,
    pub zone_id: String,
    pub x: f32,
    pub y: f32,
    pub online: bool,
}

impl Character {
    pub fn new(id: EntityId, account_id: u64, name: impl Into<String>, class: CharacterClass) -> Self {
        Self {
            id, account_id, name: name.into(), class, level: 1,
            attributes: Attributes::default(), resources: Resources::default(),
            progression: CharacterProgression { class, ..CharacterProgression::default() },
            zone_id: "starter".into(), x: 0.0, y: 0.0, online: false,
        }
    }
    pub fn grant_xp(&mut self, amount: u64) -> u32 { let before = self.progression.level; self.progression.add_xp(amount); self.level = self.progression.level; self.level.saturating_sub(before) }
    pub fn set_position(&mut self, zone: impl Into<String>, x: f32, y: f32) { self.zone_id = zone.into(); self.x = x; self.y = y; }
}
