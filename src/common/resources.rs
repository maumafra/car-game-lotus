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

pub const MARU_MONICA_FONT_PATH: &str = "fonts/x12y16pxMaruMonica.ttf";