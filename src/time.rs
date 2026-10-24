use crate::core::GameTick;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] pub struct DurationTicks(pub u64);
impl DurationTicks { pub const ZERO: Self = Self(0); pub const fn new(ticks: u64) -> Self { Self(ticks) } }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] pub struct Deadline(pub GameTick);
impl Deadline {
    pub const fn after(now: GameTick, duration: DurationTicks) -> Self { Self(GameTick(now.0.saturating_add(duration.0))) }
    pub const fn expired(self, now: GameTick) -> bool { now.0 >= self.0.0 }
    pub const fn remaining(self, now: GameTick) -> DurationTicks { DurationTicks(self.0.0.saturating_sub(now.0)) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct TickRate { pub hz: u32 }
impl Default for TickRate { fn default() -> Self { Self { hz: 20 } } }
