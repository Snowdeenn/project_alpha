mod component;
mod config;
mod eco;
mod event;
mod helper;
mod input;
mod renderer;
mod shop;
mod systems;
mod wave;
mod boost_commands;

use std::time::{Duration, Instant};

use legion::*;
use raylib::math::Vector2;
use legion::systems::CommandBuffer;

use crate::{
    component::{
        Active, Coin, CoinValue, Collider, Dash, Health, HealthState, IA, Player, Position, Shop,
        ShopItems, Velocity,
    }, config::MAX_ENEMIES, event::{DamageQueue, EnemyDiedQueue}, helper::{PlayerHealth, PlayerPos}, input::{
        InputReader,
        event::{InputQueue, InputState},
    }, renderer::{HudQueue, RenderQueue, Renderer}, shop::{ShopManager, open_close_shop_system, render_shop_system, restock_shop_system}, systems::{
        apply_damage_system, apply_pickup_system, coin_pickup_system, coin_push_to_queue_system, coin_spawn_system, collide_arena_system, collide_system, dash_system, friction_system, health_system, ia_seek_system, render_coin_system, render_hud_gold_system, render_hud_health_system, render_hud_wave_system, render_oponent_system, render_player_system, update_camera_system, update_game_state_system, update_player_pos_system, update_position_system, update_velocity_system, wave_update_system
    }, wave::{EnemyPool, WaveConfig, WaveConfigs, WaveManager, WaveState}
    
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Playing,
    Shop,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut resources = Resources::default();
    let mut renderer: Renderer = Renderer::new("Project Alpha", &resources);
    let mut world: World = World::default();
    let input_reader: InputReader = InputReader;

    resources.insert(Duration::new(0, 0));
    resources.insert(RenderQueue(vec![]));
    resources.insert(HudQueue(vec![]));
    resources.insert(InputQueue(vec![]));
    resources.insert(InputState {
        mov_dir: Vector2 { x: 0.0, y: 0.0 },
        mouse_pos: Vector2 { x: 0.0, y: 0.0 },
    });
    resources.insert(DamageQueue(vec![]));

    let mut game_schedule = Schedule::builder()
        .add_system(update_game_state_system())
        .add_system(friction_system())
        .add_system(update_velocity_system())
        .add_system(dash_system())
        .add_system(update_position_system())
        .add_system(update_player_pos_system())
        .add_system(ia_seek_system())
        .add_system(collide_system())
        .add_system(collide_arena_system())
        .add_system(apply_damage_system())
        .add_system(health_system())
        .add_system(coin_push_to_queue_system())
        .add_system(coin_spawn_system())
        .add_system(coin_pickup_system())
        .add_system(apply_pickup_system())
        .add_system(wave_update_system())
        .add_system(update_camera_system())
        .add_system(render_player_system())
        .add_system(render_oponent_system())
        .add_system(render_hud_health_system())
        .add_system(render_hud_gold_system())
        .add_system(render_hud_wave_system())
        .add_system(render_coin_system())
        .build();

    let mut shop_schedule = Schedule::builder()
        .add_system(open_close_shop_system())
        .add_system(restock_shop_system())
        .add_system(render_shop_system())
        .build();

    let mut _shop = world.push((
        Active(false),
        Shop,
        ShopItems {
            items: vec![None, None, None],
        },
    ));
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
        Health {
            hp: 100,
            state: HealthState::Alive,
        },
        Active(true),
    ));

    let mut query = <(&Position<f64>, &Health)>::query().filter(component::<Player>());
    for (pos, health) in query.iter(&world) {
        resources.insert(PlayerPos { x: pos.x, y: pos.y });
        resources.insert(PlayerHealth {
            hp: health.hp as f64,
            max_hp: 100.0,
        });
    }

    let wave_json = std::fs::read_to_string("assets/wave.json")?;
    let wave_configs: Vec<WaveConfig> = serde_json::from_str(&wave_json)?;

    // Pool d'ennemis
    let mut pool = EnemyPool { pool: Vec::new() };
    for _ in 0..MAX_ENEMIES {
        let entity = world.push((
            IA,
            Position { x: 0.0, y: 0.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Collider { w: 40.0, h: 40.0 },
            Health {
                hp: 100,
                state: HealthState::Alive,
            },
            Active(false),
        ));
        pool.pool.push(entity);
    }

    resources.insert(pool);
    resources.insert(WaveManager {
        current_wave: 0,
        enemies_remaining: wave_configs[0].enemy_count,
        enemies_to_spawn: wave_configs[0].enemy_count,
        spawn_timer: Duration::from_millis(wave_configs[0].spawn_interval),
        wave_state: WaveState::InProgress,
    });
    resources.insert(WaveConfigs(wave_configs));
    resources.insert(EnemyDiedQueue(vec![]));
    resources.insert(PlayerPos { x: 0.0, y: 0.0 });
    resources.insert(PlayerHealth {
        hp: 0.0,
        max_hp: 0.0,
    });

    resources.insert(eco::CoinSpawnQueue(vec![]));

    //todo : remplir le pool d'items du shop depuis un fichier de config

    let mut coin_pool = eco::CoinPool { coins: Vec::new() };
    for _ in 0..50 {
        let entity = world.push((
            Coin,
            Position { x: 0.0, y: 0.0 },
            Collider { w: 20.0, h: 20.0 },
            Active(false),
            CoinValue(rand::random::<u32>() % 10 + 1),
        ));
        coin_pool.coins.push(entity);
    }
    resources.insert(coin_pool);
    resources.insert(eco::PickupQueue(vec![]));
    resources.insert(eco::Gold(0));

    let game_state = GameState::Playing;
    resources.insert(game_state);

    while !renderer.rl.window_should_close() {
        let current_time: Instant = Instant::now();
        let dt: Duration = current_time - last_time;

        if let Some(mut res_dt) = resources.get_mut::<Duration>() {
            *res_dt = dt;
        }

        input_reader.update(&renderer.rl, &mut resources);
        if let Some(mut res_gm) = resources.get_mut::<GameState>() {
            *res_gm = game_state;
        }
        
        match game_state {
            GameState::Playing => {
                game_schedule.execute(&mut world, &mut resources);
            }
            GameState::Shop => {
                let mut command_buffer = CommandBuffer::new(&mut world);
                shop_schedule.execute(&mut world, &mut resources);
                ShopManager::update(&mut world, &mut resources, &mut command_buffer);
            }
        }
        
        renderer.render_frame(&mut resources);

        // fin de frame — vider les queues
        helper::clear_resource_queues(&mut resources);

        last_time = current_time;
    }

    Ok(())
}
