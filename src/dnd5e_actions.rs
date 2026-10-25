//! Turn/action economy for the game's D20 combat layer.
use serde::{Deserialize, Serialize};
use crate::dnd5e::ActionType;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionKind { Attack, CastSpell, Dash, Disengage, Dodge, Help, Hide, Ready, Search, UseItem, Grapple, Shove, Interact }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnResources { pub movement: i32, pub movement_used: i32, pub action: bool, pub bonus_action: bool, pub reaction: bool }
impl TurnResources { pub fn new(speed:i32)->Self { Self{movement:speed,movement_used:0,action:true,bonus_action:true,reaction:true} } pub fn reset(&mut self,speed:i32){self.movement=speed;self.movement_used=0;self.action=true;self.bonus_action=true;self.reaction=true;} pub fn spend(&mut self, kind:ActionType)->bool { match kind { ActionType::Action=>{if self.action{self.action=false;true}else{false}}, ActionType::BonusAction=>{if self.bonus_action{self.bonus_action=false;true}else{false}}, ActionType::Reaction=>{if self.reaction{self.reaction=false;true}else{false}}, ActionType::Movement=>false, ActionType::FreeInteraction=>true } } pub fn move_by(&mut self,distance:i32)->bool { if distance<0 || self.movement_used+distance>self.movement{return false} self.movement_used+=distance;true } }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionCommand { pub actor_id:u64, pub kind:ActionKind, pub target_id:Option<u64> }
