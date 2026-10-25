//! Spell-slot and spell-definition runtime for the game's fantasy rules.

use serde::{Deserialize, Serialize};
use crate::dnd5e::{AbilityScore, DamageType, proficiency_bonus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellSchool { Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellRange { SelfTarget, Touch, Feet(u32) }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellDefinition {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub school: SpellSchool,
    pub casting_time: String,
    pub range: SpellRange,
    pub damage_type: Option<DamageType>,
    pub base_damage_dice: Option<(u8, u8)>,
    pub concentration: bool,
    pub description: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpellSlots { pub slots: [u8; 10], pub used: [u8; 10] }

impl SpellSlots {
    pub fn available(&self, level: u8) -> u8 {
        if level as usize >= self.slots.len() { 0 } else { self.slots[level as usize].saturating_sub(self.used[level as usize]) }
    }
    pub fn spend(&mut self, level: u8) -> bool {
        let i = level as usize;
        if i >= self.slots.len() || self.used[i] >= self.slots[i] { return false; }
        self.used[i] += 1;
        true
    }
    pub fn recover_all(&mut self) { self.used = [0; 10]; }
}

pub fn spell_save_dc(ability: AbilityScore, score: i32, level: u32) -> i32 {
    8 + proficiency_bonus(level) + crate::dnd5e::AbilityScores::modifier(score) + match ability {
        AbilityScore::Strength | AbilityScore::Dexterity | AbilityScore::Constitution |
        AbilityScore::Intelligence | AbilityScore::Wisdom | AbilityScore::Charisma => 0,
    }
}

pub fn spell_attack_modifier(score: i32, level: u32) -> i32 {
    proficiency_bonus(level) + crate::dnd5e::AbilityScores::modifier(score)
}

pub fn cantrip_scaling(level: u32, base_dice: u8) -> u8 {
    let count = match level { 1..=4 => 1, 5..=10 => 2, 11..=16 => 3, _ => 4 };
    count * base_dice
}
