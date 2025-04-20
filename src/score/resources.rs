use lotus_engine::*;
use std::time::{Duration, Instant};

#[derive(Resource)]
pub struct ScoreTime {
    pub start_time: Instant,
    pub current_time: Duration
}

impl Default for ScoreTime {
    fn default() -> Self {
        return Self{
            start_time: Instant::now(),
            current_time: Duration::ZERO
        }
    }
}

#[derive(Resource)]
pub struct Highscore(pub Duration);

impl Default for Highscore {
    fn default() -> Self {
        return Self(Duration::ZERO)
    }
}