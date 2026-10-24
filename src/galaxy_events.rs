use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)] pub enum GalaxyEventKind { AlienInvasion, Supernova, RogueAiUprising, ArtifactDiscovery, FactionWar, EconomicCrisis, CosmicAnomaly }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct GalaxyEvent { pub id: String, pub kind: GalaxyEventKind, pub name: String, pub start_tick: u64, pub duration_ticks: u64, pub affected_sectors: Vec<String>, pub modifiers: Vec<String> }
impl GalaxyEvent { pub fn active_at(&self, tick: u64) -> bool { tick >= self.start_tick && tick < self.start_tick + self.duration_ticks } }
