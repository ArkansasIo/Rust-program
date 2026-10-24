use rust_battle_engine::{
    boss_runtime::{BossPhase, BossState},
    content_loader::{parse_csv, require_column},
    core::{EntityId, TeamId, Vec2},
    formulas,
    matchmaking::{MatchMode, MatchRequest, MatchmakingQueue},
    rate_limit::{RateLimit, RateLimiter},
    runtime::{GameRuntime, RuntimeConfig},
};

#[test]
fn vector_math_is_deterministic() {
    let a = Vec2 { x: 0.0, y: 0.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };
    assert_eq!(a.distance(b), 5.0);
    assert!((b.normalized().length() - 1.0).abs() < f32::EPSILON);
}

#[test]
fn formula_layer_calculates_expected_hp_and_damage() {
    assert_eq!(formulas::max_hp(10.0, 10.0, 50.0), 300.0);
    assert_eq!(formulas::physical_damage(10.0, 2.0), 50.0);
    assert!(formulas::final_damage(100.0, 0.25) > 0.0);
}

#[test]
fn matchmaking_pairs_players_by_mode() {
    let mut queue = MatchmakingQueue::default();
    queue.enqueue(MatchRequest { player_id: 1, rating: 1000, mode: MatchMode::Duel, party_size: 1 });
    queue.enqueue(MatchRequest { player_id: 2, rating: 1020, mode: MatchMode::Duel, party_size: 1 });
    assert_eq!(queue.len(MatchMode::Duel), 2);
    let ticket = queue.pop_pair(MatchMode::Duel).expect("pair should form");
    assert_eq!(ticket.requests.len(), 2);
    assert_eq!(queue.len(MatchMode::Duel), 0);
}

#[test]
fn rate_limiter_refills_on_tick() {
    let mut limiter = RateLimiter::new(RateLimit { capacity: 2, refill_per_tick: 1 });
    assert!(limiter.allow(42));
    assert!(limiter.allow(42));
    assert!(!limiter.allow(42));
    limiter.tick();
    assert!(limiter.allow(42));
}

#[test]
fn runtime_advances_ticks_and_tracks_dropped_ticks() {
    let mut runtime = GameRuntime::new(RuntimeConfig { tick_hz: 20, max_ticks_per_frame: 2 });
    runtime.step_n(5);
    assert_eq!(runtime.tick.0, 2);
    assert_eq!(runtime.metrics.ticks, 2);
    assert_eq!(runtime.metrics.dropped_ticks, 3);
}

#[test]
fn boss_transitions_to_lower_health_phase() {
    let phases = vec![
        BossPhase { phase: 1, health_threshold: 1.0, damage_multiplier: 1.0, adds: 0, mechanic: "standard".into(), arena_modifier: "none".into() },
        BossPhase { phase: 2, health_threshold: 0.5, damage_multiplier: 1.2, adds: 2, mechanic: "enrage".into(), arena_modifier: "hazard".into() },
    ];
    let mut boss = BossState::new(100.0, phases);
    assert_eq!(boss.apply_damage(60.0), 60.0);
    let phase = boss.update_phase().expect("phase transition");
    assert_eq!(phase.phase, 2);
    assert!(!boss.defeated());
}

#[test]
fn csv_loader_validates_columns() {
    let rows = parse_csv("id,role\nbrute,damage\nhealer,support").unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(require_column(&rows[0], "role").unwrap(), "damage");
    assert!(require_column(&rows[0], "missing").is_err());
}

#[test]
fn public_core_ids_are_constructible() {
    let entity = EntityId(7);
    let team = TeamId(3);
    assert_eq!(entity.0, 7);
    assert_eq!(team.0, 3);
}
