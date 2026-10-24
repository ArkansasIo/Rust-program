use crate::battle::{Battle, BattleResult};
use crate::battle::BattleConfig;
use crate::entities::Combatant;

#[derive(Debug)]
pub struct GameEngine { pub battle:Battle }
impl GameEngine { pub fn new(fighters:Vec<Combatant>)->Self{Self{battle:Battle::new(BattleConfig::default(),fighters)}} pub fn run_battle(&mut self)->BattleResult{self.battle.run()} }
