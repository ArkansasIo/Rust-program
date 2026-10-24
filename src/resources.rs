use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceKind { Hp, Mp, Stamina, Energy, Rage, Focus, Faith, Aether, Heat, Sanity }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourcePool { pub kind: ResourceKind, pub current: f32, pub maximum: f32, pub regeneration: f32 }
impl ResourcePool { pub fn spend(&mut self, amount: f32) -> bool { if self.current < amount { false } else { self.current -= amount; true } } pub fn restore(&mut self, amount: f32) { self.current = (self.current + amount).min(self.maximum); } pub fn tick(&mut self, dt: f32) { self.restore(self.regeneration * dt); } }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResourceSet { pub pools: Vec<ResourcePool> }
impl ResourceSet { pub fn get_mut(&mut self, kind: ResourceKind) -> Option<&mut ResourcePool> { self.pools.iter_mut().find(|x| x.kind == kind) } }
