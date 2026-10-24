use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::{EntityId, TeamId, Vec2};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSession { pub account_id: u64, pub character_id: EntityId, pub name: String, pub team: TeamId, pub position: Vec2, pub connected: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSnapshot { pub character_id: EntityId, pub json: String, pub version: u32 }
#[derive(Debug, Default)]
pub struct SessionManager { pub sessions: HashMap<u64, PlayerSession> }
impl SessionManager {
    pub fn connect(&mut self, session: PlayerSession) { self.sessions.insert(session.account_id, session); }
    pub fn disconnect(&mut self, account: u64) {
        if let Some(session) = self.sessions.get_mut(&account) { session.connected = false; }
    }
    pub fn online(&self) -> usize { self.sessions.values().filter(|session| session.connected).count() }
    pub fn find(&self, account: u64) -> Option<&PlayerSession> { self.sessions.get(&account) }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mail { pub id: u64, pub from: u64, pub to: u64, pub subject: String, pub body: String, pub attachments: Vec<String>, pub gold: u64, pub claimed: bool }
#[derive(Debug, Default)]
pub struct MailSystem { pub mail: HashMap<u64, Mail>, pub next_id: u64 }
impl MailSystem {
    pub fn send(&mut self, from: u64, to: u64, subject: String, body: String) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.mail.insert(id, Mail { id, from, to, subject, body, attachments: vec![], gold: 0, claimed: false });
        id
    }
    pub fn claim(&mut self, id: u64) -> bool {
        if let Some(mail) = self.mail.get_mut(&id) {
            if !mail.claimed { mail.claimed = true; return true; }
        }
        false
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchTicket { pub player: EntityId, pub mode: String, pub rating: u32 }
#[derive(Debug, Default)]
pub struct Matchmaker { pub queues: HashMap<String, Vec<MatchTicket>> }
impl Matchmaker {
    pub fn enqueue(&mut self, ticket: MatchTicket) { self.queues.entry(ticket.mode.clone()).or_default().push(ticket); }
    pub fn pop_match(&mut self, mode: &str, size: usize) -> Option<Vec<MatchTicket>> {
        if size == 0 { return None; }
        let queue = self.queues.get_mut(mode)?;
        if queue.len() < size { return None; }
        Some(queue.drain(0..size).collect())
    }
}
