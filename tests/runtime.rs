use rust_battle_engine::{GameRuntime, RuntimeConfig};

#[test]
fn runtime_advances_ticks_and_instances() {
    let mut runtime = GameRuntime::new(RuntimeConfig {
        tick_hz: 20,
        max_ticks_per_frame: 4,
    });

    runtime.step_n(3);

    assert_eq!(runtime.tick.0, 3);
    assert_eq!(runtime.metrics.ticks, 3);
    assert_eq!(runtime.metrics.dropped_ticks, 0);
}

#[test]
fn runtime_caps_frame_work() {
    let mut runtime = GameRuntime::new(RuntimeConfig {
        tick_hz: 30,
        max_ticks_per_frame: 2,
    });

    runtime.step_n(5);

    assert_eq!(runtime.tick.0, 2);
    assert_eq!(runtime.metrics.dropped_ticks, 3);
}
