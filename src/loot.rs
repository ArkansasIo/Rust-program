use crate::combat::DamageType;
use crate::items::{Item, ItemKind};

pub struct LootTable {
    pub entries: Vec<(Item, u32)>,
}

impl LootTable {
    pub fn roll(&self, seed: u64) -> Option<Item> {
        let total: u32 = self.entries.iter().map(|(_, weight)| *weight).sum();
        if total == 0 {
            return None;
        }
        let mut x = seed ^ 0x9E3779B97F4A7C15;
        x ^= x >> 30;
        x = x.wrapping_mul(0xBF58476D1CE4E5B9);
        x ^= x >> 27;
        x = x.wrapping_mul(0x94D049BB133111EB);
        x ^= x >> 31;
        let mut pick = (x as u32) % total;
        for (item, weight) in &self.entries {
            if pick < *weight {
                return Some(item.clone());
            }
            pick -= *weight;
        }
        None
    }
}

pub fn starter_loot() -> LootTable {
    LootTable {
        entries: vec![
            (
                Item {
                    id: "rust_sword".into(),
                    name: "Rust Sword".into(),
                    level: 1,
                    kind: ItemKind::Weapon {
                        damage: 8.0,
                        damage_type: DamageType::Physical,
                    },
                },
                70,
            ),
            (
                Item {
                    id: "ember_blade".into(),
                    name: "Ember Blade".into(),
                    level: 3,
                    kind: ItemKind::Weapon {
                        damage: 15.0,
                        damage_type: DamageType::Fire,
                    },
                },
                30,
            ),
        ],
    }
}
