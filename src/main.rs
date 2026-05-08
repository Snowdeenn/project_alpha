mod component;
mod helper;
mod input;
mod renderer;
mod systems;

use std::time::{Duration, Instant};

use legion::*;
use raylib::math::Vector2;

use crate::{
    component::{Collider, Dash, IA, Player, Position, Velocity},
    helper::PlayerPos,
    input::{
        InputReader,
        event::{InputQueue, InputState},
    },
    renderer::{RenderQueue, Renderer},
    systems::{
        collide_arena_system, collide_system, dash_system, friction_system, ia_seek_system,
        render_oponent_system, render_player_system, update_camera_system, update_position_system,
        update_velocity_system, update_player_pos_system,
    },
};

fn main() {
    let mut resources = Resources::default();
    let mut renderer: Renderer = Renderer::new("Project Alpha", &resources);
    let mut world: World = World::default();
    let input_reader: InputReader = InputReader;

    resources.insert(Duration::new(0, 0));
    resources.insert(RenderQueue(vec![]));
    resources.insert(InputQueue(vec![]));
    resources.insert(InputState {
        mov_dir: Vector2 { x: 0.0, y: 0.0 },
        mouse_pos: Vector2 { x: 0.0, y: 0.0 },
    });

    let mut schedule = Schedule::builder()
        .add_system(friction_system())
        .add_system(update_velocity_system())
        .add_system(dash_system())
        .add_system(update_position_system())
        .add_system(update_player_pos_system())
        .add_system(ia_seek_system())
        .add_system(collide_system())
        .add_system(collide_arena_system())
        .add_system(update_camera_system())
        .add_system(render_player_system())
        .add_system(render_oponent_system())
        .build();

    let mut last_time = Instant::now();

    let _entity_1: Entity = world.push((
        Player,
        Position {
            x: renderer.rl.get_screen_width() as f64 / 2.0,
            y: renderer.rl.get_screen_height() as f64 / 2.0,
        },
        Velocity { dx: 0.0, dy: 0.0 },
        Dash(component::DashState::Idle),
        Collider { w: 40.0, h: 40.0 },
    ));

    let _entity_temp = world.push((
        IA,
        Position { x: 800.0, y: 600.0 },
        Collider { w: 40.0, h: 40.0 },
        Velocity { dx: 0.0, dy: 0.0 },
    ));

    let mut query = <&Position<f64>>::query().filter(component::<Player>());
    for pos in query.iter(&world) {
        resources.insert(PlayerPos { x: pos.x, y: pos.y });
    }

    while !renderer.rl.window_should_close() {
        let current_time: Instant = Instant::now();
        let dt: Duration = current_time - last_time;

        if let Some(mut res_dt) = resources.get_mut::<Duration>() {
            *res_dt = dt;
        }

        input_reader.update(&renderer.rl, &mut resources);
        schedule.execute(&mut world, &mut resources);
        renderer.render_frame(&mut resources);

        // fin de frame — vider les queues
        if let Some(mut queue) = resources.get_mut::<InputQueue>() {
            queue.0.clear();
        }

        last_time = current_time;
    }
}
