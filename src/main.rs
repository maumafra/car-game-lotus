use lotus_engine::*;
use std::vec;

mod background;
mod cars;
mod common;
mod player;
mod score;

use background::resources::*;
use background::systems::*;
use cars::resources::*;
use cars::systems::*;
use common::resources::*;
use common::systems::*;
use player::systems::*;
use score::resources::*;
use score::systems::*;


your_game!(
    WindowConfiguration {
        icon_path: "sprites/64x64/cars/white-lancer.png".to_string(),
        title: "cyberlancer".to_string(),
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
        Box::new(BackgroundTileCounter(0))
    ]);
    spawn_menu(context);
    spawn_player(context);
    spawn_background_tiles(context);
    spawn_borders(context);
    spawn_score_screen(context);
}

fn update(context: &mut Context) {
    eprintln!("{:?}", context.game_loop_listener.current_fps);
    handle_input(context);
    if context.world.get_resource::<GameState>().unwrap().0 == GameStateEnum::Running {
        move_player(context);
        update_score_time(context);
        handle_background_tiles(context);
        move_background(context);
        handle_cars_movement(context);
        check_player_collisions(context);
        spawn_cars(context);
    }
}