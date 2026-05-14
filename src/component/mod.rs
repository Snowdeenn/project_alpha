use std::time::Duration;

use num_traits::Num;

pub struct Player;
pub struct IA;
pub struct Coin;

#[derive(Debug, Default)]
pub struct Position<T: Num> {
    pub(crate) x: T,
    pub(crate) y: T,
}

#[derive(Debug)]
pub struct Velocity<T: Num> {
    pub(crate) dx: T,
    pub(crate) dy: T,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum DashState {
    Idle,
    Dashing(Duration),
    Cooldown(Duration),
}
#[derive(Debug, PartialEq)]
pub struct Dash(pub DashState);

#[derive(Debug)]
pub struct Collider {
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, PartialEq)]
pub enum HealthState {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct Health {
   pub hp: u32,
   pub state: HealthState,
}

#[derive(Debug, PartialEq)]
pub struct Active(pub bool);

pub struct CoinValue(pub u32); 