use raylib::math::Vector2;

#[derive(Debug, PartialEq)]
pub enum InputEvent {
    Dash,
    Shop,
    LeftClick,
    RightClick,
}

#[derive(Debug, Default)]
pub struct InputQueue(pub Vec<InputEvent>);

#[derive(Debug, Default)]
pub struct InputState {
    pub mov_dir: Vector2,
    pub mouse_pos: Vector2,
}
