use lotus_engine::*;

#[derive(Component)]
pub struct Menu();

#[derive(Component)]
pub struct Pause();

#[derive(Component)]
pub struct PauseSelection(); // TODO Remover o resource para tratar o index por aqui

//#[derive(Component)]
//pub struct PauseSelection(pub MenuOptions)

#[derive(Component)]
pub struct GameOver();

#[derive(Component)]
pub struct GameOverSelection(pub u32);