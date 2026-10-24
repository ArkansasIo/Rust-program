use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct FriendService {
    friends: HashMap<u64, HashSet<u64>>,
    pending: HashSet<(u64, u64)>,
}
impl FriendService {
    pub fn request(&mut self, a: u64, b: u64) {
        if a != b { self.pending.insert((a, b)); }
    }
    pub fn accept(&mut self, a: u64, b: u64) -> bool {
        if self.pending.remove(&(a, b)) {
            self.friends.entry(a).or_default().insert(b);
            self.friends.entry(b).or_default().insert(a);
            true
        } else { false }
    }
    pub fn remove(&mut self, a: u64, b: u64) {
        if let Some(friends) = self.friends.get_mut(&a) { friends.remove(&b); }
        if let Some(friends) = self.friends.get_mut(&b) { friends.remove(&a); }
    }
    pub fn list(&self, a: u64) -> Vec<u64> {
        self.friends.get(&a).map(|friends| friends.iter().copied().collect()).unwrap_or_default()
    }
}
