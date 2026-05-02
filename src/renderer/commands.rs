use raylib::{color::Color};

#[derive(Debug, Clone, Copy)]
pub enum DrawCommand {
    Rectangle {x: i32, y: i32, w: i32, h: i32, color: Color },
}
