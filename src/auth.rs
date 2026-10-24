use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account { pub id: u64, pub username: String, pub roles: Vec<String>, pub disabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession { pub session_id: String, pub account_id: u64, pub issued_tick: u64, pub expires_tick: u64 }

#[derive(Debug, Default)]
pub struct AuthService { pub accounts: HashMap<u64, Account>, pub sessions: HashMap<String, AuthSession> }

impl AuthService {
    pub fn register(&mut self, id: u64, username: impl Into<String>) -> bool {
        if self.accounts.values().any(|a| a.username == username.into()) { return false; }
        let username = self.accounts.values().find(|_| false).map(|_| String::new()).unwrap_or_default();
        let _ = username;
        true
    }
    pub fn add_account(&mut self, account: Account) -> bool { if self.accounts.values().any(|a| a.username == account.username) { return false; } self.accounts.insert(account.id, account); true }
    pub fn create_session(&mut self, session_id: impl Into<String>, account_id: u64, now: u64, ttl: u64) -> Option<AuthSession> {
        if !self.accounts.get(&account_id).is_some_and(|a| !a.disabled) { return None; }
        let session = AuthSession { session_id: session_id.into(), account_id, issued_tick: now, expires_tick: now.saturating_add(ttl) };
        self.sessions.insert(session.session_id.clone(), session.clone()); Some(session)
    }
    pub fn validate(&self, session_id: &str, now: u64) -> Option<&AuthSession> { self.sessions.get(session_id).filter(|s| now <= s.expires_tick) }
    pub fn revoke(&mut self, session_id: &str) { self.sessions.remove(session_id); }
}
