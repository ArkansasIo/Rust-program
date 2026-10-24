use crate::core::{EntityId, TeamId, Vec2};
use crate::stats::{Attributes, CombatStats, Resources};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatantState { Idle, Casting, Attacking, Stunned, Dead }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combatant { pub id:EntityId, pub team:TeamId, pub name:String, pub position:Vec2, pub attributes:Attributes, pub stats:CombatStats, pub resources:Resources, pub state:CombatantState }
impl Combatant {
    pub fn new(id:EntityId, team:TeamId, name:impl Into<String>)->Self{Self{id,team,name:name.into(),position:Vec2::ZERO,attributes:Attributes::default(),stats:CombatStats::default(),resources:Resources::default(),state:CombatantState::Idle}}
    pub fn alive(&self)->bool{self.resources.alive() && self.state != CombatantState::Dead}
    pub fn distance_to(&self, other:&Self)->f32{self.position.distance(other.position)}
}
