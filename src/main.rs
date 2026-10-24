use rust_battle_engine::{Combatant, EntityId, GameEngine, TeamId, Vec2};

fn main() {
    let mut player = Combatant::new(EntityId(1), TeamId(1), "Knight");
    let mut enemy = Combatant::new(EntityId(2), TeamId(2), "Goblin");
    player.position = Vec2 { x: 0.0, y: 0.0 };
    enemy.position = Vec2 { x: 2.0, y: 0.0 };
    player.stats.attack_power = 18.0;
    enemy.stats.attack_power = 12.0;

    let mut engine = GameEngine::new(vec![player, enemy]);
    let result = engine.run_battle();

    println!("Rust Battle Engine");
    println!("==================");
    println!("Ticks: {}", result.ticks);
    println!("Combat events: {}", result.events.len());
    match result.winner {
        Some(team) => println!("Winner: Team {}", team.0),
        None => println!("Winner: draw / timeout"),
    }
}
