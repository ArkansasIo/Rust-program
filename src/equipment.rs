use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EquipmentSlot { Head, Chest, Legs, Gloves, Boots, MainHand, OffHand, Ring1, Ring2, Amulet, Cloak, Belt, Trinket1, Trinket2 }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Rarity { Common, Uncommon, Rare, Epic, Legendary, Mythic }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GearContext { Pve, Pvp, Universal }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EquipmentStats {
    pub item_power: f32, pub damage: f32, pub armor: f32, pub magic_resist: f32, pub hp: f32,
    pub crit: f32, pub haste: f32, pub resilience: f32, pub tenacity: f32, pub healing: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemData {
    pub id: String, pub name: String, pub slot: EquipmentSlot, pub rarity: Rarity,
    pub context: GearContext, pub required_level: u32, pub stats: EquipmentStats,
    pub proc_ids: Vec<String>, pub set_id: Option<String>, pub cosmetic_id: Option<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EquipmentLoadout { pub items: Vec<ItemData> }
impl EquipmentLoadout {
    pub fn equip(&mut self, item: ItemData) { self.items.retain(|x| x.slot != item.slot); self.items.push(item); }
    pub fn total_power(&self) -> f32 { self.items.iter().map(|x| x.stats.item_power).sum() }
    pub fn total(&self) -> EquipmentStats { self.items.iter().fold(EquipmentStats::default(), |mut t, x| { t.item_power += x.stats.item_power; t.damage += x.stats.damage; t.armor += x.stats.armor; t.magic_resist += x.stats.magic_resist; t.hp += x.stats.hp; t.crit += x.stats.crit; t.haste += x.stats.haste; t.resilience += x.stats.resilience; t.tenacity += x.stats.tenacity; t.healing += x.stats.healing; t }) }
}
