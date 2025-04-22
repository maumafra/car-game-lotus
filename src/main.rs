use lotus_engine::*;
use std::vec;

mod background;
mod cars;
mod common;
mod menus;
mod player;
mod score;

use background::resources::*;
use background::systems::*;
use cars::resources::*;
use cars::systems::*;
use common::resources::*;
use common::systems::*;
use menus::systems::*;
use player::systems::*;
use score::resources::*;
use score::systems::*;


your_game!(
    WindowConfiguration {
        icon_path: "sprites/64x64/cars/white-lancer.png".to_string(),
        title: "cyberlancer: made with lotus!".to_string(),
        background_color: Some(Color::BLACK),
        background_image_path: None,
        width: 960.0,
        height: 600.0,
        position_x: 200.0,
        position_y: 150.0,
        resizable: false,
        decorations: true,
        transparent: false,
        active: true,
        enabled_buttons: WindowButtons::CLOSE | WindowButtons::MINIMIZE
    },
    setup,
    update
);

fn setup(context: &mut Context) {
    context.game_loop_listener.fps_cap(144);
    context.commands.add_resources(vec![
        Box::new(GameState::default()),
        Box::new(ScoreTime::default()),
        Box::new(Highscore::default()),
        Box::new(CarSpawnTimer::default()),
        Box::new(CarSprites::default()),
        Box::new(BackgroundTileCounter(0)), // TODO change this to component
        Box::new(PauseSelectionCounter(0))  // TODO change this to component
    ]);
    spawn_main_menu(context);
    spawn_player(context);
    spawn_background_tiles(context);
    spawn_borders(context);
    spawn_score_screen(context);
    spawn_pause_menu(context);
    spawn_game_over_menu(context);
}

fn update(context: &mut Context) {
    let game_state: GameStateEnum = context.world.get_resource::<GameState>().unwrap().0.clone();
    //eprintln!("{:?}", context.game_loop_listener.current_fps);
    handle_input(context);
    if game_state == GameStateEnum::Running {
        update_score_time(context);
        handle_background_tiles(context);
        move_background(context);
        handle_cars_movement(context);
        check_player_collisions(context);
        spawn_cars(context);
    } else if game_state == GameStateEnum::GameOver {
        handle_cars_movement_on_game_over(context);
        move_crashed_player(context);
    }
}