use raylib::{color::Color};

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Rectangle {x: i32, y: i32, w: i32, h: i32, color: Color },
    Text {text: String, x: i32, y: i32, font_size: i32, color: Color},
    Circle {x: i32, y: i32, radius: i32, color: Color},
}
