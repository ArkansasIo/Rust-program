use crate::events::{EventBus, GameEvent};
use crate::network::{ClientMessage, Packet, ServerMessage};
use crate::runtime::{GameRuntime, RuntimeConfig};
use crate::shard::ShardManager;
use crate::instance::InstanceManager;

#[derive(Debug, Default)]
pub struct ServerMetrics { pub packets_received: u64, pub packets_rejected: u64, pub ticks: u64 }

#[derive(Debug)]
pub struct GameServer { pub runtime: GameRuntime, pub shards: ShardManager, pub instances: InstanceManager, pub events: EventBus, pub metrics: ServerMetrics }

impl Default for GameServer { fn default() -> Self { Self::new() } }

impl GameServer {
    pub fn new() -> Self { Self { runtime: GameRuntime::new(RuntimeConfig::default()), shards: ShardManager::default(), instances: InstanceManager::default(), events: EventBus::default(), metrics: ServerMetrics::default() } }
    pub fn tick(&mut self) { self.runtime.step(); self.metrics.ticks += 1; }
    pub fn handle_packet(&mut self, packet: Packet) -> Option<ServerMessage> {
        self.metrics.packets_received += 1;
        match packet.message {
            ClientMessage::Ping { nonce } => Some(ServerMessage::Pong { nonce }),
            ClientMessage::Login { session_id } => Some(ServerMessage::Welcome { account_id: session_id.parse().unwrap_or_default() }),
            ClientMessage::Move { .. } | ClientMessage::Attack { .. } | ClientMessage::Cast { .. } | ClientMessage::Chat { .. } | ClientMessage::Interact { .. } => None,
        }
    }
    pub fn emit(&mut self, event: GameEvent) { self.events.emit(event); }
}
