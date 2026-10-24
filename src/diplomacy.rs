use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)] pub enum DiplomaticRelation { Neutral, Trade, NonAggression, ResearchPact, Alliance, Vassal, War }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct DiplomaticAgreement { pub id: String, pub a: String, pub b: String, pub relation: DiplomaticRelation, pub expires_at: Option<i64> }
#[derive(Clone, Debug, Default, Serialize, Deserialize)] pub struct DiplomacyState { pub agreements: Vec<DiplomaticAgreement>, pub reputation: f32 }
pub fn diplomatic_score(influence: f32, reputation: f32, trade_value: f32, shared_enemies: f32) -> f32 { influence + reputation + trade_value + shared_enemies }
