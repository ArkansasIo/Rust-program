use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ShardState { Starting, Running, Draining, Offline }
#[derive(Debug, Clone)] pub struct Shard { pub id: u32, pub name: String, pub capacity: usize, pub players: usize, pub state: ShardState }
#[derive(Debug, Default)] pub struct ShardManager { pub shards: HashMap<u32, Shard> }
impl ShardManager {
    pub fn register(&mut self, shard: Shard) { self.shards.insert(shard.id, shard); }
    pub fn allocate(&self) -> Option<&Shard> { self.shards.values().filter(|s| s.state == ShardState::Running && s.players < s.capacity).min_by_key(|s| s.players) }
    pub fn connect(&mut self, shard_id: u32) -> bool { if let Some(s)=self.shards.get_mut(&shard_id) { if s.state == ShardState::Running && s.players < s.capacity { s.players+=1; return true; } } false }
    pub fn disconnect(&mut self, shard_id: u32) { if let Some(s)=self.shards.get_mut(&shard_id) { s.players=s.players.saturating_sub(1); } }
}
