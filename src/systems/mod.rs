use crate::input::event::InputState;
use crate::renderer::commands::DrawCommand;

use crate::{component::*, renderer::RenderQueue};
use legion::*;
use raylib::prelude::*;

const PLAYER_SPEED: f64 = 150.0;

#[system(for_each)]
pub fn update_position(pos: &mut Position<f64>, velo: &Velocity<f64>, #[resource] dt: &f64) {
    pos.x += velo.dx * (*dt);
    pos.y += velo.dy * (*dt);
}

#[system(for_each)]
pub fn render_player(pos: &mut Position<f64>, #[resource] queue: &mut RenderQueue) {
    queue.0.push(DrawCommand::Rectangle {
        x: pos.x as i32,
        y: pos.y as i32,
        w: 40,
        h: 40,
        color: Color::RED,
    });
}

#[system(for_each)]
#[filter(component::<Player>())]
pub fn input_consume(velo: &mut Velocity<f64>, #[resource] state: &InputState) {
    velo.dx = state.mov_dir.x as f64 * PLAYER_SPEED;
    velo.dy = state.mov_dir.y as f64 * PLAYER_SPEED;
}
