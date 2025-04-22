use lotus_engine::*;

#[derive(Component)]
pub struct Menu();

#[derive(Component)]
pub struct Pause();

#[derive(Component)]
pub struct PauseSelection(pub u32);

#[derive(Component)]
pub struct GameOver();

#[derive(Component)]
pub struct GameOverSelection(pub u32);