use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DamageElement { Physical, Fire, Ice, Lightning, Earth, Wind, Water, Light, Dark, Arcane, Poison, Bleed, Void, Chaos, True }
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Resistance { pub flat: f32, pub percent: f32, pub absorption: f32, pub vulnerability: f32, pub immune: bool }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResistanceProfile { pub physical: Resistance, pub fire: Resistance, pub ice: Resistance, pub lightning: Resistance, pub earth: Resistance, pub wind: Resistance, pub water: Resistance, pub light: Resistance, pub dark: Resistance, pub arcane: Resistance, pub poison: Resistance, pub bleed: Resistance, pub void_: Resistance, pub chaos: Resistance, pub true_: Resistance }
impl Default for ResistanceProfile { fn default() -> Self { Self { physical: Resistance::default(), fire: Resistance::default(), ice: Resistance::default(), lightning: Resistance::default(), earth: Resistance::default(), wind: Resistance::default(), water: Resistance::default(), light: Resistance::default(), dark: Resistance::default(), arcane: Resistance::default(), poison: Resistance::default(), bleed: Resistance::default(), void_: Resistance::default(), chaos: Resistance::default(), true_: Resistance::default() } } }
