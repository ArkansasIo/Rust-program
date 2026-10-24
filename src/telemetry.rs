use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Telemetry { counters: HashMap<String, u64> }
impl Telemetry {
    pub fn increment(&mut self, name: impl Into<String>, amount: u64) { *self.counters.entry(name.into()).or_default() += amount; }
    pub fn get(&self, name: &str) -> u64 { self.counters.get(name).copied().unwrap_or_default() }
    pub fn snapshot(&self) -> HashMap<String, u64> { self.counters.clone() }
}
