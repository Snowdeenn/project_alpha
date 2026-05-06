use std::time::Duration;

use crate::helper::*;
use crate::input::event::{InputEvent, InputQueue, InputState};
use crate::renderer::CameraTarget;
use crate::renderer::commands::DrawCommand;

use crate::{component::*, renderer::RenderQueue};
use legion::world::SubWorld;
use legion::*;
use num_traits::ToPrimitive;
use raylib::prelude::*;

const ACCEL: f64 = 1500.0;
// todo: Ajouter plusieurs friction en fonction
// du milieu dans lequel le joueur ce déplace.

const FRICTION: f64 = 0.85;
const ARENA_W: f64 = 1920.0;
const ARENA_H: f64 = 1080.0;

#[system(for_each)]
pub fn update_position(pos: &mut Position<f64>, velo: &Velocity<f64>, #[resource] dt: &Duration) {
    pos.x += velo.dx * (*dt).as_secs_f64();
    pos.y += velo.dy * (*dt).as_secs_f64();
}

#[system(for_each)]
#[filter(component::<Player>())]
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
#[filter(!component::<Player>())]
pub fn render_oponent(pos: &mut Position<f64>, #[resource] queue: &mut RenderQueue) {
    queue.0.push(DrawCommand::Rectangle {
        x: pos.x as i32,
        y: pos.y as i32,
        w: 40,
        h: 40,
        color: Color::BLUE,
    });
}

#[system(for_each)]
#[filter(component::<Player>())]
pub fn update_velocity(
    velo: &mut Velocity<f64>,
    #[resource] state: &InputState,
    #[resource] dt: &Duration,
) {
    let input_x = state.mov_dir.x as f64 * ACCEL * (*dt).as_secs_f64();
    let input_y = state.mov_dir.y as f64 * ACCEL * (*dt).as_secs_f64();

    velo.dx += input_x;
    velo.dy += input_y;
}

#[system(for_each)]
#[filter(component::<Player>())]
pub fn friction(velo: &mut Velocity<f64>) {
    velo.dx *= FRICTION;
    velo.dy *= FRICTION;
}

#[system(for_each)]
#[filter(component::<Player>())]
pub fn dash(
    velo: &mut Velocity<f64>,
    dash: &mut Dash,
    #[resource] queue: &InputQueue,
    #[resource] delta_time: &Duration,
) {
    let new_state = match dash.0 {
        DashState::Idle => {
            if queue.0.contains(&InputEvent::Dash) {
                velo.dx *= 7.0;
                velo.dy *= 7.0;
                DashState::Dashing(Duration::from_millis(25))
            } else {
                DashState::Idle
            }
        }

        DashState::Dashing(d) => {
            let remaining = d.saturating_sub(*delta_time);
            if remaining.is_zero() {
                DashState::Cooldown(Duration::from_secs(2))
            } else {
                DashState::Dashing(remaining)
            }
        }

        DashState::Cooldown(d) => {
            let remaining = d.saturating_sub(*delta_time);
            if remaining.is_zero() {
                DashState::Idle
            } else {
                DashState::Cooldown(remaining)
            }
        }
    };
    dash.0 = new_state;
}

#[system(for_each)]
#[filter(component::<Player>())]
pub fn update_camera(pos: &Position<f64>, #[resource] target_pos: &mut CameraTarget) {
    target_pos.pos.x = pos.x.to_f32().unwrap_or_default();
    target_pos.pos.y = pos.y.to_f32().unwrap_or_default();
}

#[system(for_each)]
pub fn collide_arena(pos: &mut Position<f64>, col: &Collider) {
    pos.x = pos.x.clamp(0.0, ARENA_W - col.w);
    pos.y = pos.y.clamp(0.0, ARENA_H - col.h);
}

#[system]
#[read_component(Position<f64>)]
#[read_component(Collider)]
#[write_component(Velocity<f64>)]
pub fn collide(world: &mut SubWorld) {
    let mut query = <(Entity, &Position<f64>, &Collider)>::query();
    let entities: Vec<_> = query.iter(world).collect();

    let mut to_resolve: Vec<Resolution> = Vec::new();
    for i in 0..entities.len() {
        for j in (i + 1)..entities.len() {
            let (ent_a, pos_a, col_a) = entities[i];
            let (ent_b, pos_b, col_b) = entities[j];

            if let Some((overlap_x, overlap_y)) = aabb_overlap(pos_a, col_a, pos_b, col_b) {
                let center_a_x = pos_a.x + col_a.w / 2.0;
                let center_b_x = pos_b.x + col_b.w / 2.0;
                let center_a_y = pos_a.y + col_a.h / 2.0;
                let center_b_y = pos_b.y + col_b.h / 2.0;

                to_resolve.push(Resolution {
                    ent_a: *ent_a,
                    ent_b: *ent_b,
                    overlap_x,
                    overlap_y,
                    dir_x: (center_a_x - center_b_x).signum(),
                    dir_y: (center_a_y - center_b_y).signum(),
                    axis: overlap_x < overlap_y,
                });
            }
        }
    }

    for res in to_resolve {
        apply_resolution(world, &res);
    }
}
