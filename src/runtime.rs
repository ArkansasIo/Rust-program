use crate::battle::Battle;
use crate::core::GameTick;
use crate::events::EventBus;
use crate::instance::InstanceManager;
use crate::shard::ShardManager;

#[derive(Debug, Clone, Copy)]
pub struct RuntimeConfig {
    pub tick_hz: u32,
    pub max_ticks_per_frame: u32,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            tick_hz: 20,
            max_ticks_per_frame: 4,
        }
    }
}

#[derive(Debug, Default)]
pub struct RuntimeMetrics {
    pub ticks: u64,
    pub dropped_ticks: u64,
    pub active_battles: u64,
}

#[derive(Debug)]
pub struct GameRuntime {
    pub config: RuntimeConfig,
    pub tick: GameTick,
    pub shards: ShardManager,
    pub instances: InstanceManager,
    pub events: EventBus,
    pub metrics: RuntimeMetrics,
}

impl Default for GameRuntime {
    fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
}

impl GameRuntime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            tick: GameTick(0),
            shards: ShardManager::default(),
            instances: InstanceManager::default(),
            events: EventBus::default(),
            metrics: RuntimeMetrics::default(),
        }
    }

    pub fn step(&mut self) {
        self.tick.0 = self.tick.0.saturating_add(1);
        self.instances.tick();
        self.metrics.ticks = self.metrics.ticks.saturating_add(1);
    }

    pub fn step_n(&mut self, ticks: u32) {
        let count = ticks.min(self.config.max_ticks_per_frame.max(1));
        for _ in 0..count {
            self.step();
        }
        self.metrics.dropped_ticks = self
            .metrics
            .dropped_ticks
            .saturating_add(u64::from(ticks.saturating_sub(count)));
    }

    pub fn register_battle(&mut self, _battle: &Battle) {
        self.metrics.active_battles = self.metrics.active_battles.saturating_add(1);
    }

    pub fn finish_battle(&mut self) {
        self.metrics.active_battles = self.metrics.active_battles.saturating_sub(1);
    }
}
