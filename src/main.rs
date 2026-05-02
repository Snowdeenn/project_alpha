mod component;
mod input;
mod renderer;
mod systems;

use legion::*;
use raylib::math::Vector2;

use crate::{
    component::{Player, Position, Velocity},
    input::{
        InputReader,
        event::{InputQueue, InputState},
    },
    renderer::{RenderQueue, Renderer},
    systems::{input_consume_system, render_player_system, update_position_system},
};

fn main() {
    let mut renderer: Renderer = Renderer::new(800, 600, "Project Alpha");
    let mut word: World = World::default();
    let input_reader: InputReader = InputReader;

    let mut resources = Resources::default();
    resources.insert(0 as f64);
    resources.insert(RenderQueue(vec![]));
    resources.insert(InputQueue(vec![]));
    resources.insert(InputState {
        mov_dir: Vector2 { x: 0.0, y: 0.0 },
        mouse_pos: Vector2 { x: 0.0, y: 0.0 },
    });

    let mut schedule = Schedule::builder()
        .add_system(update_position_system())
        .add_system(render_player_system())
        .add_system(input_consume_system())
        .build();

    let mut last_time: f64 = renderer.rl.get_time();

    let _entity_1: Entity = word.push((
        Player,
        Position { x: 400.0, y: 300.0 },
        Velocity { dx: 0.0, dy: 0.0 },
    ));

    while !renderer.rl.window_should_close() {
        let current_time: f64 = renderer.rl.get_time();
        let dt: f64 = current_time - last_time;

        if let Some(mut res_dt) = resources.get_mut::<f64>() {
            *res_dt = dt;
        }

        input_reader.update(&renderer.rl, &mut resources);
        schedule.execute(&mut word, &mut resources);
        renderer.render_frame(&mut resources);

        last_time = current_time;
    }
}
