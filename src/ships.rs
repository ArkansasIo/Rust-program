use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)] pub enum ShipClass { Fighter, Corvette, Frigate, Destroyer, Cruiser, Battleship, Carrier, Dreadnought, Titan, Flagship }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)] pub enum ComponentKind { Laser, Plasma, Railgun, Missile, IonCannon, ParticleBeam, Armor, Shield, PointDefense, Reactor, Sensors, Engines, AiCore, TargetingComputer }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct ShipComponent { pub id: String, pub kind: ComponentKind, pub power: f32, pub capacity: f32, pub accuracy: f32 }
#[derive(Clone, Debug, Serialize, Deserialize)] pub struct Ship { pub id: String, pub name: String, pub class: ShipClass, pub hull: f32, pub shields: f32, pub speed: f32, pub sensors: f32, pub supply: f32, pub crew_skill: f32, pub components: Vec<ShipComponent> }
impl Ship { pub fn combat_value(&self) -> f32 { self.hull + self.shields + self.speed * 10.0 + self.sensors * 5.0 + self.components.iter().map(|c| c.power).sum::<f32>() } }
