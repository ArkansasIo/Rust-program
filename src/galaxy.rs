use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct Galaxy { pub id: String, pub name: String, pub sectors: Vec<Sector> }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct Sector { pub id: String, pub name: String, pub systems: Vec<StarSystem> }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct StarSystem { pub id: String, pub name: String, pub planets: Vec<PlanetNode>, pub stations: Vec<String>, pub anomalies: Vec<String> }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct PlanetNode { pub id: String, pub name: String, pub habitability: f32, pub resources: Vec<String> }
impl Galaxy { pub fn system_count(&self) -> usize { self.sectors.iter().map(|s| s.systems.len()).sum() } }
