use lotus_engine::*;

use crate::menus::components::*;
use crate::common::resources::*;
use crate::common::systems::*;


const LEFT_MENU_TEXT_X: f32 = 0.425;
const LEFT_MENU_TEXT_Y: f32 = 0.05;
const MENU_DRAW_ORDER: u32 = 10;
const MENU_FONT_SIZE: f32 = 30.0;
const RIGHT_MENU_TEXT_X: f32 = 0.8;
const RIGHT_MENU_TEXT_Y: f32 = 0.17;
const PAUSE_OPTIONS_LEN: i32 = 2;
const GAME_OVER_OPTIONS_LEN: i32 = 3;

pub fn change_menu_visibility<T: Component>(context: &Context) {
    let pause_entities: Vec<Entity> = {
        let mut query: Query =  Query::new(&context.world).with::<T>();
        query.entities_with_components().unwrap()
    };
    for entity in pause_entities {
        change_visibilty(context, &entity);
    }
}

pub fn enter_game_over_option(context: &mut Context) {
    let game_state: GameStateEnum = context.world.get_resource::<GameState>().unwrap().0.clone();
    if game_state != GameStateEnum::GameOver {
        return;
    }
    let go_selection_entity: Entity = {
        let mut query: Query = Query::new(&context.world).with::<GameOverSelection>();
        query.entities_with_components().unwrap().first().unwrap().clone()
    };
    let selection: u32 = {
        let selection_counter: ComponentRef<'_, GameOverSelection> = context.world.get_entity_component(&go_selection_entity).unwrap();
        selection_counter.0
    };
    if selection == 0 {
        change_menu_visibility::<GameOver>(context);
        reset_game(context);
        retry(context);
    } else if selection == 1 {
        change_menu_visibility::<GameOver>(context);
        change_menu_visibility::<Menu>(context);
        reset_game(context);
        quit_to_menu(context);
    } else if selection == 2 {
        quit_game();
    }
}

pub fn enter_pause_option(context: &mut Context) {
    let game_state: GameStateEnum = context.world.get_resource::<GameState>().unwrap().0.clone();
    if game_state != GameStateEnum::Paused {
        return;
    }
    let selection: u32 = context.world.get_resource_mut::<PauseSelectionCounter>().unwrap().0;
    if selection == 0 {
        change_menu_visibility::<Pause>(context);
        change_menu_visibility::<Menu>(context);
        reset_game(context);
        quit_to_menu(context);
    } else if selection == 1 {
        quit_game();
    }
}

pub fn move_game_over_selection(context: &mut Context, direction: i32) {
    let game_state: GameStateEnum = context.world.get_resource::<GameState>().unwrap().0.clone();
    if game_state != GameStateEnum::GameOver {
        return;
    }
    let go_selection_entity: Entity = {
        let mut query: Query = Query::new(&context.world).with::<GameOverSelection>();
        query.entities_with_components().unwrap().first().unwrap().clone()
    };
    let selection: u32 = {
        let mut selection_counter: ComponentRefMut<'_, GameOverSelection> = context.world.get_entity_component_mut(&go_selection_entity).unwrap();
        selection_counter.0 = ((selection_counter.0 as i32 + direction)%GAME_OVER_OPTIONS_LEN).unsigned_abs();
        selection_counter.0
    };

    let y: f32 = RIGHT_MENU_TEXT_Y + 0.07*selection as f32;
    let text_renderer: &mut TextRenderer =  context.world.text_renderers.get_mut(&go_selection_entity.0).unwrap();
    text_renderer.text.position.update_values(Vector2::new(RIGHT_MENU_TEXT_X-0.02, y));
    if selection == 0 {
        text_renderer.text.color = Color::YELLOW;
    } else {
        text_renderer.text.color = Color::WHITE;
    }
}

pub fn move_pause_selection(context: &mut Context, direction: i32) {
    let game_state: GameStateEnum = context.world.get_resource::<GameState>().unwrap().0.clone();
    if game_state != GameStateEnum::Paused {
        return;
    }
    let pause_selection_entity: Entity = {
        let mut query: Query = Query::new(&context.world).with::<PauseSelection>();
        query.entities_with_components().unwrap().first().unwrap().clone()
    };
    let selection: u32 = {
        let mut selection_counter: ResourceRefMut<'_, PauseSelectionCounter> = context.world.get_resource_mut::<PauseSelectionCounter>().unwrap();
        selection_counter.0 = ((selection_counter.0 as i32 + direction)%PAUSE_OPTIONS_LEN).unsigned_abs();
        selection_counter.0
    };

    let y: f32 = RIGHT_MENU_TEXT_Y + 0.07 + 0.07*selection as f32;
    context.world.text_renderers.get_mut(&pause_selection_entity.0).unwrap().text.position.update_values(Vector2::new(RIGHT_MENU_TEXT_X-0.02, y));
}

pub fn spawn_main_menu(context: &mut Context) {
    let start_text: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), MENU_FONT_SIZE),
        Position::new(Vector2::new(LEFT_MENU_TEXT_X, LEFT_MENU_TEXT_Y), Strategy::Pixelated),
        Color::YELLOW,
        "> enter <".to_string()
    );
    let start_text_shadow: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), MENU_FONT_SIZE),
        Position::new(Vector2::new(LEFT_MENU_TEXT_X+0.002, LEFT_MENU_TEXT_Y+0.002), Strategy::Pixelated),
        Color::BLACK,
        "> enter <".to_string()
    );
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/960x600/menu.png".to_string())),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
                0.0,
                Scale::new(Vector2::new(1.6, 1.0), Strategy::Normalized))),
            Box::new(Menu()),
            Box::new(DrawOrder(MENU_DRAW_ORDER))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(start_text_shadow),
            Box::new(Menu()),
            Box::new(DrawOrder(MENU_DRAW_ORDER+1))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(start_text),
            Box::new(Menu()),
            Box::new(DrawOrder(MENU_DRAW_ORDER+2))
        ]
    );
}

pub fn spawn_pause_menu(context: &mut Context) {
    spawn_right_menu(context);
    let pause: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 40.0),
        Position::new(Vector2::new(RIGHT_MENU_TEXT_X, 0.07), Strategy::Pixelated),
        Color::WHITE,
        "pause".to_string()
    );
    let selection: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 30.0),
        Position::new(Vector2::new(RIGHT_MENU_TEXT_X-0.02, RIGHT_MENU_TEXT_Y+0.07), Strategy::Pixelated),
        Color::WHITE,
        ">".to_string()
    );
    context.commands.spawn(
        vec![
            Box::new(pause),
            Box::new(Pause()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(selection),
            Box::new(Pause()),
            Box::new(PauseSelection()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER))
        ]
    );
}

pub fn spawn_game_over_menu(context: &mut Context) {
    spawn_right_menu(context);
    let game_over: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 40.0),
        Position::new(Vector2::new(RIGHT_MENU_TEXT_X, 0.07), Strategy::Pixelated),
        Color::WHITE,
        "game over :(".to_string()
    );
    let retry: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 30.0),
        Position::new(Vector2::new(RIGHT_MENU_TEXT_X, RIGHT_MENU_TEXT_Y), Strategy::Pixelated),
        Color::YELLOW,
        "retry :)".to_string()
    );
    let selection: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 30.0),
        Position::new(Vector2::new(RIGHT_MENU_TEXT_X-0.02, RIGHT_MENU_TEXT_Y), Strategy::Pixelated),
        Color::YELLOW,
        ">".to_string()
    );
    context.commands.spawn(
        vec![
            Box::new(game_over),
            Box::new(GameOver()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(retry),
            Box::new(GameOver()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(selection),
            Box::new(GameOver()),
            Box::new(GameOverSelection(0)),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER))
        ]
    );
}

// This spawns the right corner menu and options, which are common to both game over and pause menus
fn spawn_right_menu(context: &mut Context) {
    let quit_to_menu: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 30.0),
        Position::new(Vector2::new(RIGHT_MENU_TEXT_X, RIGHT_MENU_TEXT_Y + 0.07), Strategy::Pixelated),
        Color::WHITE,
        "quit to menu".to_string()
    );
    let quit: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 30.0),
        Position::new(Vector2::new(RIGHT_MENU_TEXT_X, RIGHT_MENU_TEXT_Y + 0.14), Strategy::Pixelated),
        Color::WHITE,
        "quit game".to_string()
    );
    context.commands.spawn(
        vec![
            Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK)),
            Box::new(Transform::new(
                Position::new(Vector2::new(1.3, 0.0), Strategy::Normalized),
                0.0,
                Scale::new(Vector2::new(0.6, 4.0), Strategy::Normalized))),
            Box::new(Pause()),
            Box::new(GameOver()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER-1)),
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(quit_to_menu),
            Box::new(Pause()),
            Box::new(GameOver()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(quit),
            Box::new(Pause()),
            Box::new(GameOver()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER))
        ]
    );
}