use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage { Login { session_id: String }, Move { x: f32, y: f32 }, Attack { target: u64, ability: String }, Cast { ability: String, target: Option<u64> }, Chat { channel: String, text: String }, Interact { entity: u64 }, Ping { nonce: u64 } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage { Welcome { account_id: u64 }, Snapshot { tick: u64, payload: String }, CombatEvent { payload: String }, Chat { channel: String, sender: String, text: String }, Error { code: String, message: String }, Pong { nonce: u64 } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet { pub sequence: u64, pub message: ClientMessage }

#[derive(Debug, Default)]
pub struct SequenceGuard { last: u64 }
impl SequenceGuard { pub fn accept(&mut self, sequence: u64) -> bool { if sequence <= self.last { false } else { self.last = sequence; true } } }
