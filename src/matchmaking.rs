use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchMode { Duel, Arena, Dungeon, Raid, Battleground, FleetBattle }

#[derive(Debug, Clone)]
pub struct MatchRequest { pub player_id: u64, pub rating: i32, pub mode: MatchMode, pub party_size: u8 }

#[derive(Debug, Clone)]
pub struct MatchTicket { pub id: u64, pub requests: Vec<MatchRequest> }

#[derive(Debug, Default)]
pub struct MatchmakingQueue { queues: HashMap<MatchMode, VecDeque<MatchRequest>>, next_ticket: u64 }

impl MatchmakingQueue {
    pub fn enqueue(&mut self, request: MatchRequest) { self.queues.entry(request.mode).or_default().push_back(request); }
    pub fn pop_pair(&mut self, mode: MatchMode) -> Option<MatchTicket> {
        let queue = self.queues.get_mut(&mode)?;
        if queue.len() < 2 { return None; }
        let a = queue.pop_front()?; let b = queue.pop_front()?;
        self.next_ticket += 1;
        Some(MatchTicket { id: self.next_ticket, requests: vec![a, b] })
    }
    pub fn len(&self, mode: MatchMode) -> usize { self.queues.get(&mode).map_or(0, VecDeque::len) }
}
