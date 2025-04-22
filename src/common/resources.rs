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
pub struct GameAudio(pub AudioSource);

impl Default for GameAudio {
    fn default() -> Self {
        return Self(AudioSource::new().expect("Should create a audio source."));
    }
}

pub const MARU_MONICA_FONT_PATH: &str = "fonts/x12y16pxMaruMonica.ttf";
pub const HEIGHT: f32 = 600.0;
pub const WIDTH: f32 = 960.0;