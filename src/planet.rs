use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)] pub enum BuildingType { MiningComplex, ResearchLab, Shipyard, TradeHub, DefenseGrid, OrbitalCannon, HabitatDome }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct Building { pub id: String, pub kind: BuildingType, pub level: u32, pub production: f64, pub upkeep: f64 }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct Planet { pub id: String, pub name: String, pub size: u32, pub habitability: f32, pub population: f64, pub resource_output: f64, pub defense: f32, pub stability: f32, pub buildings: Vec<Building> }
impl Planet { pub fn net_output(&self) -> f64 { self.resource_output + self.buildings.iter().map(|b| b.production).sum::<f64>() - self.buildings.iter().map(|b| b.upkeep).sum::<f64>() } }
