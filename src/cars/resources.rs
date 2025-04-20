use lotus_engine::*;
use std::time::Duration;

#[derive(Resource)]
pub struct CarSpawnTimer(pub Timer);

impl Default for CarSpawnTimer {
    fn default() -> Self {
        return Self(Timer::new(TimerType::Repeat, Duration::new(1, 0)))
    }
}

#[derive(Clone, Resource)]
pub struct CarSprites(pub Vec<String>);

impl Default for CarSprites {
    fn default() -> Self {
        return Self(Vec::from(vec![
            "sprites/64x64/cars/red-car.png".to_string(),
            "sprites/64x64/cars/blue-car.png".to_string(),
            "sprites/64x64/cars/yellow-car.png".to_string()
        ]))
    }
}