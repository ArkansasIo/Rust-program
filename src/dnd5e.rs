//! D&D 5e-inspired rules engine for the new game.
//!
//! This module implements original game-facing mechanics inspired by the
//! familiar d20 fantasy framework. It does not reproduce proprietary rulebook
//! text. Content and names used here should be treated as game data that can
//! be replaced by the project's own setting.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AbilityScore {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl AbilityScore {
    pub const ALL: [Self; 6] = [
        Self::Strength,
        Self::Dexterity,
        Self::Constitution,
        Self::Intelligence,
        Self::Wisdom,
        Self::Charisma,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

impl Default for AbilityScores {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
}

impl AbilityScores {
    pub fn get(&self, ability: AbilityScore) -> i32 {
        match ability {
            AbilityScore::Strength => self.strength,
            AbilityScore::Dexterity => self.dexterity,
            AbilityScore::Constitution => self.constitution,
            AbilityScore::Intelligence => self.intelligence,
            AbilityScore::Wisdom => self.wisdom,
            AbilityScore::Charisma => self.charisma,
        }
    }

    pub fn modifier(score: i32) -> i32 {
        (score - 10).div_euclid(2)
    }

    pub fn modifier_for(&self, ability: AbilityScore) -> i32 {
        Self::modifier(self.get(ability))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProficiencyTier {
    None,
    Proficient,
    Expertise,
}

impl ProficiencyTier {
    pub fn bonus(self, level: u32) -> i32 {
        let level = level.clamp(1, 20) as i32;
        let proficiency = 2 + ((level - 1) / 4);
        match self {
            Self::None => 0,
            Self::Proficient => proficiency,
            Self::Expertise => proficiency * 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResult {
    pub natural_d20: u8,
    pub total: i32,
    pub dc: i32,
    pub success: bool,
    pub critical_success: bool,
    pub critical_failure: bool,
}

pub fn resolve_d20(natural_d20: u8, modifier: i32, dc: i32) -> CheckResult {
    let roll = natural_d20.clamp(1, 20);
    let total = roll as i32 + modifier;
    CheckResult {
        natural_d20: roll,
        total,
        dc,
        success: total >= dc,
        critical_success: roll == 20,
        critical_failure: roll == 1,
    }
}

pub fn passive_score(ability_modifier: i32, proficiency: ProficiencyTier, level: u32) -> i32 {
    10 + ability_modifier + proficiency.bonus(level)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdvantageState {
    Normal,
    Advantage,
    Disadvantage,
}

pub fn choose_d20(a: u8, b: u8, state: AdvantageState) -> u8 {
    let a = a.clamp(1, 20);
    let b = b.clamp(1, 20);
    match state {
        AdvantageState::Normal => a,
        AdvantageState::Advantage => a.max(b),
        AdvantageState::Disadvantage => a.min(b),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Action,
    BonusAction,
    Reaction,
    Movement,
    FreeInteraction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
    Acid,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Poison,
    Psychic,
    Radiant,
    Thunder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SavingThrow {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreatureSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterClass {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl CharacterClass {
    pub const ALL: [Self; 12] = [
        Self::Barbarian, Self::Bard, Self::Cleric, Self::Druid,
        Self::Fighter, Self::Monk, Self::Paladin, Self::Ranger,
        Self::Rogue, Self::Sorcerer, Self::Warlock, Self::Wizard,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    Orc,
    Tiefling,
    Dragonkin,
    Gnome,
    HalfElf,
    HalfOrc,
    Aasimar,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    Blinded,
    Charmed,
    Deafened,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
    Exhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HitDie {
    D6,
    D8,
    D10,
    D12,
}

impl HitDie {
    pub fn sides(self) -> i32 {
        match self {
            Self::D6 => 6,
            Self::D8 => 8,
            Self::D10 => 10,
            Self::D12 => 12,
        }
    }
}

pub fn proficiency_bonus(level: u32) -> i32 {
    2 + ((level.clamp(1, 20) - 1) / 4) as i32
}

pub fn initiative(dexterity_modifier: i32, bonuses: i32) -> i32 {
    dexterity_modifier + bonuses
}

pub fn armor_class(base: i32, dexterity_modifier: i32, armor_bonus: i32, shield_bonus: i32) -> i32 {
    base + dexterity_modifier + armor_bonus + shield_bonus
}

pub fn hit_points(level: u32, hit_die: HitDie, constitution_modifier: i32, rolled_hp: Option<i32>) -> i32 {
    let level = level.clamp(1, 20);
    let first = hit_die.sides() + constitution_modifier;
    let per_level = rolled_hp.unwrap_or(hit_die.sides() / 2 + 1) + constitution_modifier;
    (first + (level.saturating_sub(1) as i32 * per_level)).max(1)
}

pub fn spell_save_dc(spellcasting_modifier: i32, level: u32) -> i32 {
    8 + proficiency_bonus(level) + spellcasting_modifier
}

pub fn spell_attack_bonus(spellcasting_modifier: i32, level: u32) -> i32 {
    proficiency_bonus(level) + spellcasting_modifier
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSheet {
    pub id: u64,
    pub name: String,
    pub race: Race,
    pub class: CharacterClass,
    pub level: u32,
    pub experience: u64,
    pub alignment: Alignment,
    pub abilities: AbilityScores,
    pub hit_die: HitDie,
    pub max_hp: i32,
    pub current_hp: i32,
    pub armor_class: i32,
    pub speed: i32,
    pub proficiency: Vec<String>,
    pub conditions: Vec<Condition>,
}

impl CharacterSheet {
    pub fn new(id: u64, name: impl Into<String>, class: CharacterClass, race: Race) -> Self {
        let abilities = AbilityScores::default();
        let hit_die = match class {
            CharacterClass::Barbarian => HitDie::D12,
            CharacterClass::Fighter | CharacterClass::Paladin | CharacterClass::Ranger => HitDie::D10,
            _ => HitDie::D8,
        };
        let hp = hit_points(1, hit_die, abilities.modifier_for(AbilityScore::Constitution), None);
        Self {
            id,
            name: name.into(),
            race,
            class,
            level: 1,
            experience: 0,
            alignment: Alignment::TrueNeutral,
            abilities,
            hit_die,
            max_hp: hp,
            current_hp: hp,
            armor_class: 10 + abilities.modifier_for(AbilityScore::Dexterity),
            speed: 30,
            proficiency: Vec::new(),
            conditions: Vec::new(),
        }
    }

    pub fn proficiency_bonus(&self) -> i32 {
        proficiency_bonus(self.level)
    }

    pub fn ability_modifier(&self, ability: AbilityScore) -> i32 {
        self.abilities.modifier_for(ability)
    }

    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    pub fn apply_damage(&mut self, amount: i32) -> i32 {
        let dealt = amount.max(0).min(self.current_hp);
        self.current_hp -= dealt;
        dealt
    }

    pub fn heal(&mut self, amount: i32) -> i32 {
        let healed = amount.max(0).min(self.max_hp - self.current_hp);
        self.current_hp += healed;
        healed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ability_modifier_uses_d20_style_rounding() {
        assert_eq!(AbilityScores::modifier(10), 0);
        assert_eq!(AbilityScores::modifier(12), 1);
        assert_eq!(AbilityScores::modifier(9), -1);
        assert_eq!(AbilityScores::modifier(8), -1);
    }

    #[test]
    fn d20_criticals_are_detected() {
        assert!(resolve_d20(20, 0, 30).critical_success);
        assert!(resolve_d20(1, 20, 1).critical_failure);
    }

    #[test]
    fn character_can_take_damage_and_heal() {
        let mut character = CharacterSheet::new(1, "Aris", CharacterClass::Fighter, Race::Human);
        let hp = character.max_hp;
        assert_eq!(character.apply_damage(5), 5);
        assert_eq!(character.heal(2), 2);
        assert_eq!(character.current_hp, hp - 3);
    }
}
