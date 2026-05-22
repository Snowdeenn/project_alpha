use raylib::color::Color;

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Rectangle { x: i32, y: i32, w: i32, h: i32, color: Color },
    Text { text: String, x: i32, y: i32, font_size: i32, color: Color },
    Circle { x: i32, y: i32, radius: i32, color: Color },
}

// Commandes HUD — coordonnées écran, fonte custom
#[derive(Debug, Clone)]
pub enum HudCommand {
    Rectangle { x: i32, y: i32, w: i32, h: i32, color: Color },
    Text { text: String, x: i32, y: i32, font_size: i32, spacing: f32, color: Color },
    Circle { x: i32, y: i32, radius: f32, color: Color },
}