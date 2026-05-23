use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ShopItem {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub effect_type: EffectType,
    pub effect_value: f64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum EffectType {
    #[default]
    Health,
    Damage,
    Speed,
    Gold,
}

pub struct ItemPool {
    pub items: Vec<ShopItem>,
}