//! Class progression data for the game's D&D 5e-inspired character system.

use serde::{Deserialize, Serialize};
use crate::dnd5e::{AbilityScore, CharacterClass, HitDie};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellcastingKind { None, Prepared, Known, Pact }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDefinition {
    pub class: CharacterClass,
    pub hit_die: HitDie,
    pub primary_abilities: Vec<AbilityScore>,
    pub saving_throws: Vec<AbilityScore>,
    pub spellcasting: SpellcastingKind,
    pub armor_training: Vec<String>,
    pub weapon_training: Vec<String>,
    pub starting_features: Vec<String>,
}

pub fn class_definition(class: CharacterClass) -> ClassDefinition {
    use AbilityScore::*;
    use CharacterClass::*;
    match class {
        Barbarian => ClassDefinition { class, hit_die: HitDie::D12, primary_abilities: vec![Strength], saving_throws: vec![Strength, Constitution], spellcasting: SpellcastingKind::None, armor_training: vec!["light".into(), "medium".into(), "shields".into()], weapon_training: vec!["simple".into(), "martial".into()], starting_features: vec!["rage".into(), "unarmored_defense".into()] },
        Bard => ClassDefinition { class, hit_die: HitDie::D8, primary_abilities: vec![Charisma], saving_throws: vec![Dexterity, Charisma], spellcasting: SpellcastingKind::Known, armor_training: vec!["light".into()], weapon_training: vec!["simple".into()], starting_features: vec!["bardic_inspiration".into(), "spellcasting".into()] },
        Cleric => ClassDefinition { class, hit_die: HitDie::D8, primary_abilities: vec![Wisdom], saving_throws: vec![Wisdom, Charisma], spellcasting: SpellcastingKind::Prepared, armor_training: vec!["light".into(), "medium".into(), "shields".into()], weapon_training: vec!["simple".into()], starting_features: vec!["divine_domain".into(), "spellcasting".into()] },
        Druid => ClassDefinition { class, hit_die: HitDie::D8, primary_abilities: vec![Wisdom], saving_throws: vec![Intelligence, Wisdom], spellcasting: SpellcastingKind::Prepared, armor_training: vec!["light".into(), "medium".into(), "shields".into()], weapon_training: vec!["simple".into()], starting_features: vec!["druidic".into(), "spellcasting".into()] },
        Fighter => ClassDefinition { class, hit_die: HitDie::D10, primary_abilities: vec![Strength, Dexterity], saving_throws: vec![Strength, Constitution], spellcasting: SpellcastingKind::None, armor_training: vec!["light".into(), "medium".into(), "heavy".into(), "shields".into()], weapon_training: vec!["simple".into(), "martial".into()], starting_features: vec!["fighting_style".into(), "second_wind".into()] },
        Monk => ClassDefinition { class, hit_die: HitDie::D8, primary_abilities: vec![Dexterity, Wisdom], saving_throws: vec![Strength, Dexterity], spellcasting: SpellcastingKind::None, armor_training: vec![], weapon_training: vec!["simple".into()], starting_features: vec!["martial_arts".into(), "unarmored_movement".into()] },
        Paladin => ClassDefinition { class, hit_die: HitDie::D10, primary_abilities: vec![Strength, Charisma], saving_throws: vec![Wisdom, Charisma], spellcasting: SpellcastingKind::Prepared, armor_training: vec!["light".into(), "medium".into(), "heavy".into(), "shields".into()], weapon_training: vec!["simple".into(), "martial".into()], starting_features: vec!["divine_sense".into(), "lay_on_hands".into()] },
        Ranger => ClassDefinition { class, hit_die: HitDie::D10, primary_abilities: vec![Dexterity, Wisdom], saving_throws: vec![Strength, Dexterity], spellcasting: SpellcastingKind::Prepared, armor_training: vec!["light".into(), "medium".into(), "shields".into()], weapon_training: vec!["simple".into(), "martial".into()], starting_features: vec!["favored_enemy".into(), "exploration".into()] },
        Rogue => ClassDefinition { class, hit_die: HitDie::D8, primary_abilities: vec![Dexterity], saving_throws: vec![Dexterity, Intelligence], spellcasting: SpellcastingKind::None, armor_training: vec!["light".into()], weapon_training: vec!["simple".into()], starting_features: vec!["expertise".into(), "sneak_attack".into()] },
        Sorcerer => ClassDefinition { class, hit_die: HitDie::D6, primary_abilities: vec![Charisma], saving_throws: vec![Constitution, Charisma], spellcasting: SpellcastingKind::Known, armor_training: vec![], weapon_training: vec!["simple".into()], starting_features: vec!["spellcasting".into(), "sorcerous_origin".into()] },
        Warlock => ClassDefinition { class, hit_die: HitDie::D8, primary_abilities: vec![Charisma], saving_throws: vec![Wisdom, Charisma], spellcasting: SpellcastingKind::Pact, armor_training: vec!["light".into()], weapon_training: vec!["simple".into()], starting_features: vec!["otherworldly_patron".into(), "pact_magic".into()] },
        Wizard => ClassDefinition { class, hit_die: HitDie::D6, primary_abilities: vec![Intelligence], saving_throws: vec![Intelligence, Wisdom], spellcasting: SpellcastingKind::Prepared, armor_training: vec![], weapon_training: vec!["simple".into()], starting_features: vec!["spellbook".into(), "spellcasting".into()] },
    }
}

pub fn all_class_definitions() -> Vec<ClassDefinition> {
    CharacterClass::ALL.into_iter().map(class_definition).collect()
}
