pub const SCREEN_H: i32 = 1080;
pub const SCREEN_W: i32 = 1920;
pub const MAX_ENEMIES: usize = 100;

use raylib::color::Color;
// --- CONFIGURATION BOUTIQUE (DIMENSIONS & POSITIONS) ---
pub const SHOP_OVERLAY_COLOR: Color = Color::new(0, 0, 0, 150);

pub const SHOP_TITLE_TEXT: &str = "BOUTIQUE INTER-VAGUE";
pub const SHOP_TITLE_X: i32 = 700;
pub const SHOP_TITLE_Y: i32 = 120;
pub const SHOP_TITLE_FONT_SIZE: i32 = 64;
pub const SHOP_TITLE_SPACING: f32 = 2.0;

pub const SHOP_CARD_Y: i32 = 290;
pub const SHOP_CARD_W: i32 = 350;
pub const SHOP_CARD_H: i32 = 500;
pub const SHOP_SLOTS_X: [i32; 3] = [335, 785, 1235];

// --- ÉLÉMENTS INTERNES DES CARTES ACTIVES ---
pub const SHOP_BORDER_OFFSET: i32 = 5;
pub const SHOP_ART_OFFSET_X: i32 = 40;
pub const SHOP_ART_OFFSET_Y: i32 = 40;
pub const SHOP_ART_W: i32 = 270;
pub const SHOP_ART_H: i32 = 200;

pub const SHOP_TEXT_PADDING_X: i32 = 20;
pub const SHOP_NAME_OFFSET_Y: i32 = 260;
pub const SHOP_NAME_FONT_SIZE: i32 = 32;
pub const SHOP_TEXT_SPACING: f32 = 1.0;

pub const SHOP_PRICE_OFFSET_Y: i32 = 430;
pub const SHOP_PRICE_FONT_SIZE: i32 = 28;

// --- CARTES VENDUES ---
pub const SHOP_SOLD_BG_COLOR: Color = Color::new(40, 40, 40, 255);
pub const SHOP_SOLD_TEXT: &str = "VENDU";
pub const SHOP_SOLD_TEXT_OFFSET_X: i32 = 120;
pub const SHOP_SOLD_TEXT_OFFSET_Y: i32 = 230;
pub const SHOP_SOLD_FONT_SIZE: i32 = 40;
pub const SHOP_SOLD_SPACING: f32 = 1.5;