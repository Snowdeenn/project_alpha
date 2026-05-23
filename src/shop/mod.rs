use crate::GameState;
use crate::boost_commands::{ApplyDamage, ApplyGold, ApplyHealth, ApplySpeed};
use crate::component::*;
use crate::component::{Active, Shop};
use crate::eco::Gold;
use crate::input::event::{InputEvent, InputQueue, InputState};
use crate::shop::helper::EffectType;
use crate::shop::helper::ItemPool;
use legion::systems::CommandBuffer;
use legion::*;
use rand::prelude::IndexedRandom;

pub mod helper;

#[system(for_each)]
#[filter(component::<Shop>())]
pub fn open_close(
    entity: &Entity,
    active: &mut Active,
    #[resource] i_queue: &InputQueue,
    #[resource] game_state: &mut GameState,
    commands: &mut CommandBuffer,
) {
    if i_queue.0.contains(&InputEvent::Shop) && *game_state == GameState::Playing {
        active.0 = true;
        *game_state = GameState::Shop;
        commands.add_component(*entity, NeedRestock);
    } else if i_queue.0.contains(&InputEvent::Shop) && *game_state == GameState::Shop {
        active.0 = false;
        *game_state = GameState::Playing;
    }
}

#[system(for_each)]
#[filter(component::<Shop>() & component::<NeedRestock>())]
pub fn restock_shop(
    entity: &Entity,
    commands: &mut CommandBuffer,
    shop_items: &mut ShopItems,
    #[resource] item_pool: &ItemPool,
) {
    let mut rng = rand::rng();

    let mut nombre_items_shop = 3;
    if item_pool.items.len() < 3 {
        nombre_items_shop = item_pool.items.len();
    }

    let random_items = item_pool
        .items
        .sample(&mut rng, nombre_items_shop)
        .cloned()
        .map(Some)
        .collect();

    shop_items.items = random_items;

    commands.remove_component::<NeedRestock>(*entity);
    println!(
        "Le shop a été réapprovisionné avec {} items aléatoires !",
        nombre_items_shop
    );
}

use crate::renderer::HudQueue;
use crate::renderer::commands::HudCommand;
use raylib::color::Color;
use crate::config::*;

#[system]
pub fn render_shop(
    world: &legion::world::SubWorld,
    #[resource] hud_queue: &mut HudQueue,
    shop_query: &mut Query<(&Active, &ShopItems)>,
) {
    // 1. Fond semi-transparent
    hud_queue.0.push(HudCommand::Rectangle {
        x: 0,
        y: 0,
        w: SCREEN_W,
        h: SCREEN_H,
        color: SHOP_OVERLAY_COLOR,
    });

    // 2. Titre de la boutique
    hud_queue.0.push(HudCommand::Text {
        text: SHOP_TITLE_TEXT.to_string(),
        x: SHOP_TITLE_X,
        y: SHOP_TITLE_Y,
        font_size: SHOP_TITLE_FONT_SIZE,
        spacing: SHOP_TITLE_SPACING,
        color: Color::GOLD,
    });

    if let Some((_, shop_items)) = shop_query.iter(world).next() {
        for slot in 0..3 {
            let x = SHOP_SLOTS_X[slot];

            match shop_items.items.get(slot) {
                Some(Some(item)) => {
                    let card_color = match item.effect_type {
                        EffectType::Health => Color::DARKGREEN,
                        EffectType::Damage => Color::MAROON,
                        EffectType::Speed => Color::DARKBLUE,
                        EffectType::Gold => Color::GOLD,
                    };

                    // Fond de la carte
                    hud_queue.0.push(HudCommand::Rectangle {
                        x,
                        y: SHOP_CARD_Y,
                        w: SHOP_CARD_W,
                        h: SHOP_CARD_H,
                        color: card_color,
                    });
                    
                    // Bordure interne noire
                    hud_queue.0.push(HudCommand::Rectangle {
                        x: x + SHOP_BORDER_OFFSET,
                        y: SHOP_CARD_Y + SHOP_BORDER_OFFSET,
                        w: SHOP_CARD_W - (SHOP_BORDER_OFFSET * 2),
                        h: SHOP_CARD_H - (SHOP_BORDER_OFFSET * 2),
                        color: Color::BLACK,
                    });
                    
                    // Cadre d'illustration
                    hud_queue.0.push(HudCommand::Rectangle {
                        x: x + SHOP_ART_OFFSET_X,
                        y: SHOP_CARD_Y + SHOP_ART_OFFSET_Y,
                        w: SHOP_ART_W,
                        h: SHOP_ART_H,
                        color: Color::DARKGRAY,
                    });

                    // Nom de l'item
                    hud_queue.0.push(HudCommand::Text {
                        text: item.name.clone(),
                        x: x + SHOP_TEXT_PADDING_X,
                        y: SHOP_CARD_Y + SHOP_NAME_OFFSET_Y,
                        font_size: SHOP_NAME_FONT_SIZE,
                        spacing: SHOP_TEXT_SPACING,
                        color: Color::WHITE,
                    });
                    
                    // Prix de l'item
                    hud_queue.0.push(HudCommand::Text {
                        text: format!("PRIX: {} OR", item.price),
                        x: x + SHOP_TEXT_PADDING_X,
                        y: SHOP_CARD_Y + SHOP_PRICE_OFFSET_Y,
                        font_size: SHOP_PRICE_FONT_SIZE,
                        spacing: SHOP_TEXT_SPACING,
                        color: Color::GOLD,
                    });
                }
                Some(None) => {
                    // Rendu de la carte vendue
                    hud_queue.0.push(HudCommand::Rectangle {
                        x,
                        y: SHOP_CARD_Y,
                        w: SHOP_CARD_W,
                        h: SHOP_CARD_H,
                        color: SHOP_SOLD_BG_COLOR,
                    });
                    
                    hud_queue.0.push(HudCommand::Text {
                        text: SHOP_SOLD_TEXT.to_string(),
                        x: x + SHOP_SOLD_TEXT_OFFSET_X,
                        y: SHOP_CARD_Y + SHOP_SOLD_TEXT_OFFSET_Y,
                        font_size: SHOP_SOLD_FONT_SIZE,
                        spacing: SHOP_SOLD_SPACING,
                        color: Color::GRAY,
                    });
                }
                None => (),
            }
        }
    }
}

pub struct ShopManager;

impl ShopManager {
    pub fn update(
        world: &mut World,
        resources: &mut Resources,
        commands: &mut CommandBuffer,
    ) {
        let i_queue = resources.get::<InputQueue>().unwrap();
        
        if !i_queue.0.contains(&InputEvent::LeftClick) {
            return;
        }

        let mouse_pos = resources.get::<InputState>().unwrap().mouse_pos;

        if !(mouse_pos.y >= SHOP_CARD_Y as f32 && mouse_pos.y <= (SHOP_CARD_Y + SHOP_CARD_H) as f32) {
            return;
        }

        let clicked_slot = match mouse_pos.x as i32 {
            x if x >= SHOP_SLOTS_X[0] && x <= (SHOP_SLOTS_X[0] + SHOP_CARD_W) => Some(0),
            x if x >= SHOP_SLOTS_X[1] && x <= (SHOP_SLOTS_X[1] + SHOP_CARD_W) => Some(1),
            x if x >= SHOP_SLOTS_X[2] && x <= (SHOP_SLOTS_X[2] + SHOP_CARD_W) => Some(2),
            _ => None,
        };

        let Some(slot) = clicked_slot else { return };

        let item = {
            let mut query = <(&Shop, &mut ShopItems)>::query();
            query.iter_mut(world)
                .next()
                .and_then(|(_, items)| items.items.get(slot).and_then(|i| i.clone()))
        };

        let Some(item) = item else { return };

        let has_gold = resources.get::<Gold>().map(|g| g.0 >= item.price).unwrap_or(false);
        if !has_gold { return };

        resources.get_mut::<Gold>().unwrap().0 -= item.price;

        let mut player_query = <(Entity, &Player)>::query();
        let player_entity = player_query.iter(world).next().map(|(e, _)| *e);

        if let Some(player) = player_entity {
            match item.effect_type {
                EffectType::Health => {
                    commands.add_component(player, ApplyHealth(item.effect_value));
                }
                EffectType::Speed => {
                    commands.add_component(player, ApplySpeed(item.effect_value));
                }
                EffectType::Damage => {
                    commands.add_component(player, ApplyDamage(item.effect_value));
                }
                EffectType::Gold => {
                    commands.add_component(player, ApplyGold(item.effect_value as u32));
                }
            }
        }

        let mut query = <(&Shop, &mut ShopItems)>::query();
        if let Some((_, items)) = query.iter_mut(world).next() {
            if let Some(slot_ref) = items.items.get_mut(slot) {
                slot_ref.take();
            }
        }

        println!("Achat de '{}' pour {} or !", item.name, item.price);
    }
}
