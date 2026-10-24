use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct RateLimit { pub capacity: u32, pub refill_per_tick: u32 }

#[derive(Debug, Clone, Copy)]
struct Bucket { tokens: u32 }

#[derive(Debug)]
pub struct RateLimiter { rule: RateLimit, buckets: HashMap<u64, Bucket> }

impl RateLimiter {
    pub fn new(rule: RateLimit) -> Self { Self { rule, buckets: HashMap::new() } }
    pub fn allow(&mut self, key: u64) -> bool {
        let bucket = self.buckets.entry(key).or_insert(Bucket { tokens: self.rule.capacity });
        if bucket.tokens == 0 { false } else { bucket.tokens -= 1; true }
    }
    pub fn tick(&mut self) { for bucket in self.buckets.values_mut() { bucket.tokens = (bucket.tokens + self.rule.refill_per_tick).min(self.rule.capacity); } }
}
