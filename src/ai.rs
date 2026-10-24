use crate::entities::Combatant;
use crate::core::EntityId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiState { Idle, Chase, Attack, Flee, Dead }
#[derive(Debug, Clone)]
pub struct AiController { pub state:AiState, pub attack_range:f32 }
impl Default for AiController { fn default()->Self{Self{state:AiState::Idle,attack_range:2.5}} }
impl AiController {
    pub fn choose_target<'a>(&self,actor:&Combatant,candidates:&'a [Combatant])->Option<&'a Combatant>{candidates.iter().filter(|x|x.alive()&&x.team!=actor.team).min_by(|a,b|actor.distance_to(a).partial_cmp(&actor.distance_to(b)).unwrap_or(std::cmp::Ordering::Equal))}
    pub fn think(&mut self,actor:&Combatant,target:Option<&Combatant>)->Option<EntityId>{let Some(target)=target else {self.state=AiState::Idle;return None};let d=actor.distance_to(target);self.state=if d<=self.attack_range{AiState::Attack}else{AiState::Chase};Some(target.id)}
}
