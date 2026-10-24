use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct CoreAttributes {
    pub str_: f32,
    pub dex: f32,
    pub con: f32,
    pub int_: f32,
    pub wis: f32,
    pub vit: f32,
    pub spi: f32,
    pub lck: f32,
    pub wil: f32,
    pub cha: f32,
}

impl CoreAttributes {
    pub fn total(&self) -> f32 { self.str_ + self.dex + self.con + self.int_ + self.wis + self.vit + self.spi + self.lck + self.wil + self.cha }
    pub fn scaled(&self, factor: f32) -> Self { Self { str_: self.str_ * factor, dex: self.dex * factor, con: self.con * factor, int_: self.int_ * factor, wis: self.wis * factor, vit: self.vit * factor, spi: self.spi * factor, lck: self.lck * factor, wil: self.wil * factor, cha: self.cha * factor } }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct SubAttributes {
    pub physical_power: f32,
    pub armor_penetration: f32,
    pub attack_speed: f32,
    pub accuracy: f32,
    pub evasion: f32,
    pub weapon_handling: f32,
    pub crit_damage: f32,
    pub spell_power: f32,
    pub elemental_mastery: f32,
    pub cast_speed: f32,
    pub magic_penetration: f32,
    pub mana_efficiency: f32,
    pub overcharge: f32,
    pub hp_regen: f32,
    pub shield_strength: f32,
    pub damage_reduction: f32,
    pub block_chance: f32,
    pub parry_chance: f32,
}

pub fn derive_sub_attributes(a: CoreAttributes) -> SubAttributes {
    SubAttributes {
        physical_power: a.str_ * 2.0 + a.dex * 0.5,
        armor_penetration: a.str_ * 0.2 + a.dex * 0.1,
        attack_speed: a.dex * 0.04,
        accuracy: a.dex * 0.5,
        evasion: a.dex * 0.35,
        weapon_handling: a.dex * 0.25,
        crit_damage: 150.0 + a.dex * 0.25 + a.lck * 0.1,
        spell_power: a.int_ * 2.0 + a.wis * 0.5,
        elemental_mastery: a.int_ * 0.4 + a.spi * 0.3,
        cast_speed: a.int_ * 0.03,
        magic_penetration: a.int_ * 0.2,
        mana_efficiency: a.wis * 0.15,
        overcharge: a.spi * 0.2,
        hp_regen: a.con * 0.2 + a.vit * 0.1,
        shield_strength: a.con * 0.2 + a.wis * 0.1,
        damage_reduction: a.con * 0.15 + a.vit * 0.1,
        block_chance: a.con * 0.05,
        parry_chance: a.dex * 0.05,
    }
}
