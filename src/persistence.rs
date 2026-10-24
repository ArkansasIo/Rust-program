use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

pub trait Repository<K, V> {
    fn load(&self, key: &K) -> Option<V>;
    fn save(&mut self, key: K, value: V);
    fn delete(&mut self, key: &K) -> bool;
}

#[derive(Debug, Default)]
pub struct MemoryRepository<K, V> { pub rows: HashMap<K, V> }
impl<K: Eq + std::hash::Hash + Clone, V: Clone> Repository<K, V> for MemoryRepository<K, V> {
    fn load(&self, key: &K) -> Option<V> { self.rows.get(key).cloned() }
    fn save(&mut self, key: K, value: V) { self.rows.insert(key, value); }
    fn delete(&mut self, key: &K) -> bool { self.rows.remove(key).is_some() }
}

pub fn encode_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> { serde_json::to_string_pretty(value) }
pub fn decode_json<T: DeserializeOwned>(text: &str) -> Result<T, serde_json::Error> { serde_json::from_str(text) }
