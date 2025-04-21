use lotus_engine::*;

#[derive(Clone, Resource)]
pub struct GameState(pub GameStateEnum);

impl Default for GameState {
    fn default() -> Self {
        return Self(GameStateEnum::Menu);
    }
}

#[derive(Clone, PartialEq)]
pub enum GameStateEnum {
    Menu,
    Paused,
    Running,
    GameOver
}

#[derive(Resource)]
pub struct PauseSelectionCounter(pub u32);