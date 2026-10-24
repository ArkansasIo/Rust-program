use crate::combat::{AttackKind, CombatEvent, CombatSystem, DamageType};
use crate::core::{EntityId, GameTick, TeamId};
use crate::entities::{Combatant, CombatantState};
use crate::events::{EventBus, GameEvent};

#[derive(Debug, Clone, Copy)]
pub struct BattleConfig {
    pub max_ticks: u64,
    pub basic_attack_range: f32,
    pub basic_damage: f32,
}
impl Default for BattleConfig {
    fn default() -> Self {
        Self { max_ticks: 1000, basic_attack_range: 2.5, basic_damage: 10.0 }
    }
}
#[derive(Debug, Clone)]
pub struct BattleResult { pub winner: Option<TeamId>, pub ticks: u64, pub events: Vec<CombatEvent> }

#[derive(Debug)]
pub struct Battle {
    pub config: BattleConfig,
    pub tick: GameTick,
    pub fighters: Vec<Combatant>,
    pub events: EventBus,
}
impl Battle {
    pub fn new(config: BattleConfig, fighters: Vec<Combatant>) -> Self {
        Self { config, tick: GameTick(0), fighters, events: EventBus::default() }
    }

    /// Resolves each tick from a snapshot: fighters alive at tick start can act
    /// during that tick, so attacks are simultaneous rather than index-ordered.
    pub fn step(&mut self) -> Vec<CombatEvent> {
        let snapshot = self.fighters.clone();
        let mut emitted = Vec::new();

        for i in 0..snapshot.len() {
            if !snapshot[i].alive() { continue; }
            let target = (0..snapshot.len())
                .filter(|&j| snapshot[j].alive() && snapshot[j].team != snapshot[i].team)
                .min_by(|&a, &b| snapshot[i].distance_to(&snapshot[a])
                    .partial_cmp(&snapshot[i].distance_to(&snapshot[b]))
                    .unwrap_or(std::cmp::Ordering::Equal));
            let Some(j) = target else { continue; };
            if snapshot[i].distance_to(&snapshot[j]) > self.config.basic_attack_range { continue; }

            let event = CombatSystem::calculate_damage(
                &snapshot[i], &snapshot[j], self.config.basic_damage, DamageType::Physical, false,
            );
            emitted.push(event);
        }

        // Apply the collected events only after every attack has been calculated.
        for event in &emitted {
            if let Some(target) = self.fighters.iter_mut().find(|f| f.id == event.target) {
                CombatSystem::apply(event, target);
            }
            self.events.emit(GameEvent::Combat(event.clone()));
        }
        for fighter in &mut self.fighters {
            if fighter.alive() && emitted.iter().any(|event| event.source == fighter.id) {
                fighter.state = CombatantState::Attacking;
            } else if fighter.alive() && fighter.state == CombatantState::Attacking {
                fighter.state = CombatantState::Idle;
            }
        }
        self.tick.0 += 1;
        emitted
    }

    pub fn run(&mut self) -> BattleResult {
        let mut all = Vec::new();
        while self.tick.0 < self.config.max_ticks {
            all.extend(self.step());
            if let Some(winner) = self.winner() {
                return BattleResult { winner: Some(winner), ticks: self.tick.0, events: all };
            }
            if self.fighters.iter().all(|f| !f.alive()) { break; }
        }
        BattleResult { winner: self.winner(), ticks: self.tick.0, events: all }
    }
    pub fn winner(&self) -> Option<TeamId> {
        let teams: Vec<_> = self.fighters.iter().filter(|f| f.alive()).map(|f| f.team).collect();
        teams.first().copied().filter(|team| teams.iter().all(|candidate| candidate == team))
    }
    pub fn fighter(&self, id: EntityId) -> Option<&Combatant> {
        self.fighters.iter().find(|fighter| fighter.id == id)
    }
}

#[allow(dead_code)]
fn _attack_kind_used(_: AttackKind) {}
