#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum BehaviorStatus { Success, Running, Failure }
#[derive(Debug, Clone)] pub enum BehaviorNode { Sequence(Vec<BehaviorNode>), Selector(Vec<BehaviorNode>), Action(u32), Wait(u32) }
impl BehaviorNode { pub fn action(id:u32)->Self{Self::Action(id)} pub fn sequence(nodes:Vec<Self>)->Self{Self::Sequence(nodes)} pub fn selector(nodes:Vec<Self>)->Self{Self::Selector(nodes)} }
#[derive(Debug,Default)] pub struct BehaviorState { pub cursor:usize, pub elapsed:u32 }
impl BehaviorState { pub fn reset(&mut self){self.cursor=0;self.elapsed=0;} }
