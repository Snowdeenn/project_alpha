use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShopItem {
    pub name: String,
    pub description: String, // Nouvelle donnée pour ton HUD !
    pub price: u32,
    pub rarity: Rarity,      // Nouvelle donnée pour la couleur de la carte !
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