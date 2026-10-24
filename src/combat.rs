use crate::core::EntityId;
use crate::entities::Combatant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageType { Physical, Fire, Ice, Lightning, Poison, Holy, Shadow, True }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackKind { Basic, Skill, Critical, Dot }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEvent { pub source:EntityId, pub target:EntityId, pub kind:AttackKind, pub damage_type:DamageType, pub raw_damage:f32, pub final_damage:f32, pub critical:bool }

#[derive(Debug, Default)]
pub struct CombatSystem;
impl CombatSystem {
    pub fn calculate_damage(source:&Combatant,target:&Combatant,base:f32,damage_type:DamageType,critical:bool)->CombatEvent {
        let scaled=base + source.stats.attack_power;
        let crit=if critical {scaled*source.stats.crit_multiplier} else {scaled};
        let mitigation=match damage_type { DamageType::True=>0.0, DamageType::Physical=>target.stats.armor.max(0.0), _=>target.stats.magic_resist.max(0.0) };
        let multiplier=if damage_type==DamageType::True {1.0} else {100.0/(100.0+mitigation)};
        let final_damage=(crit*multiplier).max(0.0);
        CombatEvent{source:source.id,target:target.id,kind:if critical{AttackKind::Critical}else{AttackKind::Basic},damage_type,raw_damage:crit,final_damage,critical}
    }
    pub fn apply(event:&CombatEvent,target:&mut Combatant){ if target.alive(){target.resources.hp-=event.final_damage;target.resources.clamp();if target.resources.hp<=0.0{target.state=crate::entities::CombatantState::Dead;}} }
}
