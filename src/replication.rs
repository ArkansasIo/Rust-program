use crate::core::EntityId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDelta { pub entity_id: EntityId, pub changed_fields: HashMap<String, String> }

#[derive(Debug, Default)]
pub struct ReplicationState { known: HashMap<u64, HashMap<EntityId, EntityDelta>> }
impl ReplicationState {
    pub fn update(&mut self, player_id: u64, delta: EntityDelta) { self.known.entry(player_id).or_default().insert(delta.entity_id, delta); }
    pub fn snapshot(&self, player_id: u64) -> Vec<EntityDelta> { self.known.get(&player_id).map(|m| m.values().cloned().collect()).unwrap_or_default() }
    pub fn clear_player(&mut self, player_id: u64) { self.known.remove(&player_id); }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationOp { Spawn { entity: EntityId }, Despawn { entity: EntityId }, Update { entity: EntityId, component: String, payload: String } }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Snapshot { pub tick: u64, pub operations: Vec<ReplicationOp> }

#[derive(Debug, Default)]
pub struct SnapshotBuffer { pub snapshots: Vec<Snapshot> }
impl SnapshotBuffer {
    pub fn push(&mut self, snapshot: Snapshot) { self.snapshots.push(snapshot); }
    pub fn latest(&self) -> Option<&Snapshot> { self.snapshots.last() }
    pub fn prune_before(&mut self, tick: u64) { self.snapshots.retain(|s| s.tick >= tick); }
}
