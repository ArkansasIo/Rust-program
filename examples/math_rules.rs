//! Example of using the expression language for a small rule calculation.
use rust_environment_api::language::{evaluate, Value};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let variables = HashMap::from([
        ("strength".to_owned(), Value::Integer(18)),
        ("vitality".to_owned(), Value::Integer(14)),
        ("alive".to_owned(), Value::Boolean(true)),
    ]);

    let max_hp = evaluate("vitality * 15 + strength * 2", &variables)?;
    let can_fight = evaluate("alive && vitality > 0", &variables)?;
    println!("Calculated derived value: {max_hp}");
    println!("Can fight: {can_fight}");
    Ok(())
}
