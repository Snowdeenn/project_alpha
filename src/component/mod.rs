use std::time::Duration;

use num_traits::Num;

pub struct Player;

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

