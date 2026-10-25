use rust_battle_engine::dnd5e::*;
use rust_battle_engine::dnd5e_actions::*;
use rust_battle_engine::dnd5e_character::*;
use rust_battle_engine::dnd5e_skills::*;
use rust_battle_engine::dnd5e_rules::*;
#[test]
fn modifiers_and_proficiency_work(){assert_eq!(AbilityScores::modifier(8),-1);assert_eq!(proficiency_bonus(1),2);assert_eq!(proficiency_bonus(20),6);}
#[test]
fn skill_resolution_uses_skill_ability(){let s=AbilityScores{strength:16,..Default::default()};let r=resolve_skill(&s,Skill::Athletics,ProficiencyTier::Proficient,1,10,14);assert_eq!(r.total,15);assert!(r.success);}
#[test]
fn action_resources_reset_and_spend(){let mut t=TurnResources::new(30);assert!(t.spend(ActionType::Action));assert!(!t.spend(ActionType::Action));assert!(t.move_by(20));assert!(!t.move_by(20));t.reset(30);assert!(t.spend(ActionType::Action));}
#[test]
fn character_builder_creates_valid_character(){let c=CharacterBuild{name:"Test Hero".into(),race:Race::Human,class:CharacterClass::Fighter,abilities:AbilityScores{strength:16,..Default::default()},background_id:None,feat_ids:vec![]}.create(42);assert_eq!(c.id,42);assert!(c.max_hp>0);assert!(c.is_alive());}
#[test]
fn passive_checks_handle_advantage(){assert_eq!(passive(3,ProficiencyTier::Proficient,1,AdvantageState::Normal),15);assert_eq!(passive(3,ProficiencyTier::Proficient,1,AdvantageState::Advantage),20);}
