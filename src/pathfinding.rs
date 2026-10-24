use std::collections::{HashMap, HashSet, VecDeque};
use crate::core::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct GridPos { pub x: i32, pub y: i32 }
impl GridPos { pub fn neighbors(self) -> [Self; 4] { [Self{x:self.x+1,y:self.y},Self{x:self.x-1,y:self.y},Self{x:self.x,y:self.y+1},Self{x:self.x,y:self.y-1}] } pub fn distance(self, other: Self) -> u32 { self.x.abs_diff(other.x)+self.y.abs_diff(other.y) } }
#[derive(Debug, Default)] pub struct GridMap { blocked: HashSet<GridPos> }
impl GridMap {
    pub fn set_blocked(&mut self, pos: GridPos, blocked: bool) { if blocked { self.blocked.insert(pos); } else { self.blocked.remove(&pos); } }
    pub fn is_walkable(&self, pos: GridPos) -> bool { !self.blocked.contains(&pos) }
    pub fn find_path(&self, start: GridPos, goal: GridPos, max_nodes: usize) -> Option<Vec<GridPos>> { if !self.is_walkable(start)||!self.is_walkable(goal){return None;} let mut q=VecDeque::from([start]); let mut parent=HashMap::new(); let mut seen=HashSet::from([start]); while let Some(cur)=q.pop_front(){ if cur==goal { let mut p=vec![cur]; let mut c=cur; while let Some(&v)=parent.get(&c){p.push(v);c=v;} p.reverse(); return Some(p);} if seen.len()>=max_nodes.max(1){break;} for n in cur.neighbors(){if self.is_walkable(n)&&seen.insert(n){parent.insert(n,cur);q.push_back(n);}} } None }
}
pub fn grid_to_world(pos: GridPos, cell_size: f32) -> Vec2 { Vec2::new(pos.x as f32*cell_size,pos.y as f32*cell_size) }
