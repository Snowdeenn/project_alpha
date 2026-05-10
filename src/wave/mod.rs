use std::time::Duration;

use serde::Deserialize;

#[derive(Debug)]
pub struct WaveManager {
    pub current_wave: usize,      // usize pour indexer directement dans wave_configs
    pub enemies_remaining: u32,
    pub enemies_to_spawn: u32,    // ennemis pas encore spawnés cette vague
    pub spawn_timer: Duration,
    pub wave_state: WaveState,
}

#[derive(Debug)]
pub enum WaveState {
    InProgress,
    BetweenWave(Duration),
}

#[derive(Debug, Deserialize)]
pub struct WaveConfig {
    pub enemy_count: u32, 
    pub enemy_hp: u32,
    pub enemy_speed: f64,
    pub spawn_interval: u64,
}

#[derive(Debug)]
pub struct WaveConfigs(pub Vec<WaveConfig>);
