use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Attributes { pub strength:u32, pub dexterity:u32, pub constitution:u32, pub intelligence:u32, pub wisdom:u32, pub vitality:u32, pub spirit:u32, pub luck:u32 }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CombatStats { pub attack_power:f32, pub spell_power:f32, pub armor:f32, pub magic_resist:f32, pub crit_chance:f32, pub crit_multiplier:f32, pub attack_speed:f32, pub move_speed:f32 }
impl Default for CombatStats { fn default()->Self{Self{attack_power:10.0,spell_power:10.0,armor:0.0,magic_resist:0.0,crit_chance:0.05,crit_multiplier:2.0,attack_speed:1.0,move_speed:5.0}} }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Resources { pub hp:f32, pub max_hp:f32, pub mana:f32, pub max_mana:f32, pub stamina:f32, pub max_stamina:f32 }
impl Resources { pub fn alive(&self)->bool{self.hp>0.0} pub fn clamp(&mut self){self.hp=self.hp.clamp(0.0,self.max_hp);self.mana=self.mana.clamp(0.0,self.max_mana);self.stamina=self.stamina.clamp(0.0,self.max_stamina);} }
impl Default for Resources { fn default()->Self{Self{hp:100.0,max_hp:100.0,mana:100.0,max_mana:100.0,stamina:100.0,max_stamina:100.0}} }
