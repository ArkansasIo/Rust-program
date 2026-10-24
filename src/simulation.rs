use crate::core::GameTick;

pub trait SimSystem { fn name(&self) -> &'static str; fn tick(&mut self, tick: GameTick); }

#[derive(Default)]
pub struct SimulationPipeline { systems: Vec<Box<dyn SimSystem>> }
impl std::fmt::Debug for SimulationPipeline { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("SimulationPipeline").field("systems", &self.system_names()).finish() } }
impl SimulationPipeline {
    pub fn add<S: SimSystem + 'static>(&mut self, system: S) { self.systems.push(Box::new(system)); }
    pub fn tick(&mut self, tick: GameTick) { for system in &mut self.systems { system.tick(tick); } }
    pub fn system_names(&self) -> Vec<&'static str> { self.systems.iter().map(|s| s.name()).collect() }
    pub fn len(&self) -> usize { self.systems.len() }
}
