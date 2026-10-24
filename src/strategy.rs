use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)] pub enum EmpireAiStyle { Expansionist, Defensive, Economic, Aggressive, Technological, Diplomatic }
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)] pub struct StrategicSignals { pub threat_level: f32, pub resource_need: f32, pub expansion_opportunity: f32, pub diplomatic_risk: f32 }
pub fn priority(s: StrategicSignals) -> f32 { s.threat_level + s.resource_need + s.expansion_opportunity + s.diplomatic_risk }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)] pub enum VictoryCondition { Military, Economic, Technological, Diplomatic, Ascension }
