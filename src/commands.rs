use crate::core::{EntityId, GameTick};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCommand {
    Move { entity: EntityId, x: i32, y: i32 },
    Attack { attacker: EntityId, target: EntityId },
    UseAbility { entity: EntityId, ability: u32, target: Option<EntityId> },
    Interact { entity: EntityId, target: EntityId },
    UseItem { entity: EntityId, item: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueuedCommand { pub tick: GameTick, pub sequence: u64, pub command: GameCommand }

#[derive(Debug, Default)]
pub struct CommandQueue { next_sequence: u64, commands: Vec<QueuedCommand> }
impl CommandQueue {
    pub fn push(&mut self, tick: GameTick, command: GameCommand) { let sequence = self.next_sequence; self.next_sequence = self.next_sequence.saturating_add(1); self.commands.push(QueuedCommand { tick, sequence, command }); }
    pub fn drain_for_tick(&mut self, tick: GameTick) -> Vec<QueuedCommand> { let mut ready = Vec::new(); let mut pending = Vec::with_capacity(self.commands.len()); for c in self.commands.drain(..) { if c.tick.0 <= tick.0 { ready.push(c); } else { pending.push(c); } } self.commands = pending; ready.sort_by_key(|c| c.sequence); ready }
    pub fn len(&self) -> usize { self.commands.len() }
    pub fn is_empty(&self) -> bool { self.commands.is_empty() }
}
