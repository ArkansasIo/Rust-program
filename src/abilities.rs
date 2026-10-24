use crate::combat::{CombatEvent, CombatSystem, DamageType};
use crate::entities::Combatant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability { pub id:String, pub name:String, pub power:f32, pub damage_type:DamageType, pub range:f32, pub cooldown_ticks:u64, pub resource_cost:f32 }
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cooldowns { pub entries:std::collections::HashMap<String,u64> }
impl Cooldowns { pub fn ready(&self,id:&str)->bool{self.entries.get(id).copied().unwrap_or(0)==0} pub fn tick(&mut self){for v in self.entries.values_mut(){*v=v.saturating_sub(1);}} }
impl Ability {
    pub fn execute(&self,source:&mut Combatant,target:&mut Combatant,cooldowns:&mut Cooldowns)->Option<CombatEvent>{
        if !cooldowns.ready(&self.id) || source.resources.mana < self.resource_cost || !source.alive() || !target.alive() || source.distance_to(target)>self.range {return None;}
        source.resources.mana-=self.resource_cost;
        cooldowns.entries.insert(self.id.clone(),self.cooldown_ticks);
        let event=CombatSystem::calculate_damage(source,target,self.power,self.damage_type,false);
        CombatSystem::apply(&event,target); Some(event)
    }
}
