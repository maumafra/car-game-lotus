use lotus_engine::*;

use crate::common::components::*;
use crate::common::resources::*;
use crate::background::systems::reset_background;
use crate::cars::systems::reset_cars;
use crate::player::systems::move_player;
use crate::player::systems::reset_player;
use crate::score::systems::reset_score;
use crate::score::systems::resume_score;
use crate::score::systems::start_score;

pub const MARU_MONICA_FONT_PATH: &str = "fonts/x12y16pxMaruMonica.ttf";
const MENU_FONT_SIZE: f32 = 30.0;
const MENU_TEXT_X: f32 = 0.425;
const MENU_TEXT_Y: f32 = 0.05;
const MENU_DRAW_ORDER: u32 = 10;
const PAUSE_TEXT_X: f32 = 0.8;
const PAUSE_TEXT_Y: f32 = 0.24;

pub fn handle_input(context: &mut Context) {
    let input: Input = {
        context.world.get_resource::<Input>().unwrap().clone()
    };

    if input.is_key_released(PhysicalKey::Code(KeyCode::Enter)) {
        start_game(context);
        select_pause_option(context);
    }

    if input.is_key_released(PhysicalKey::Code(KeyCode::Escape)) {
        toggle_pause(context);
    }

    if input.is_key_released(PhysicalKey::Code(KeyCode::Semicolon)) {
        set_debug_visibility(context);
    }

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyA)) {
        move_player(context, -1.0);
    }
    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyD)) {
        move_player(context, 1.0);
    }
    if input.is_key_released(PhysicalKey::Code(KeyCode::KeyA))
    || input.is_key_released(PhysicalKey::Code(KeyCode::KeyD))
    || (input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyA))
    && input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyD))) {
        move_player(context, 0.0);
    }

    if input.is_key_released(PhysicalKey::Code(KeyCode::KeyW)) {
        move_pause_selection(context, -1);
    }
    if input.is_key_released(PhysicalKey::Code(KeyCode::KeyS)) {
        move_pause_selection(context, 1);
    }
}

fn select_pause_option(context: &mut Context) {
    let game_state: GameStateEnum = context.world.get_resource::<GameState>().unwrap().0.clone();
    if game_state != GameStateEnum::Paused {
        return;
    }
    let selection: u32 = context.world.get_resource_mut::<PauseSelectionCounter>().unwrap().0;
    if selection == 0 {
        quit_menu(context);
        reset_game(context);
    } else if selection == 1 {
        quit_game();
    }
}

fn quit_game() {
    std::process::exit(0);
}

fn reset_game(context: &mut Context) {
    reset_background(context);
    reset_score(context);
    reset_player(context);
    reset_cars(context);
}

fn quit_menu(context: &Context) {
    let menu_entities: Vec<Entity> = {
        let mut query: Query =  Query::new(&context.world).with::<Menu>();
        query.entities_with_components().unwrap()
    };
    for entity in menu_entities {
        change_visibilty(context, &entity);
    }
    let pause_entities: Vec<Entity> = {
        let mut query: Query =  Query::new(&context.world).with::<Pause>();
        query.entities_with_components().unwrap()
    };
    for entity in pause_entities {
        change_visibilty(context, &entity);
    }
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    game_state.0 = GameStateEnum::Menu;
}

fn move_pause_selection(context: &mut Context, direction: i32) {
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
        selection_counter.0 = ((selection_counter.0 as i32 + direction)%2).unsigned_abs();
        selection_counter.0
    };

    let y: f32 = PAUSE_TEXT_Y + 0.07*selection as f32;
    context.world.text_renderers.get_mut(&pause_selection_entity.0).unwrap().text.position.update_values(Vector2::new(PAUSE_TEXT_X-0.02, y));
}

fn start_game(context: &Context) {
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    if game_state.0 == GameStateEnum::Menu {
        game_state.0 = GameStateEnum::Running;
        let menu_entities: Vec<Entity> = {
            let mut menu_query: Query =  Query::new(&context.world).with::<Menu>();
            menu_query.entities_with_components().unwrap()
        };
        for entity in menu_entities {
            change_visibilty(context, &entity);
        }
        start_score(context);
    }
}

fn toggle_pause(context: &Context) {
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    if game_state.0 == GameStateEnum::Running {
        game_state.0 = GameStateEnum::Paused;

        let pause_entities: Vec<Entity> = {
            let mut query: Query =  Query::new(&context.world).with::<Pause>();
            query.entities_with_components().unwrap()
        };
        for entity in pause_entities {
            change_visibilty(context, &entity);
        }
    } else if game_state.0 == GameStateEnum::Paused {
        game_state.0 = GameStateEnum::Running;
        resume_score(context);

        let pause_entities: Vec<Entity> = {
            let mut query: Query =  Query::new(&context.world).with::<Pause>();
            query.entities_with_components().unwrap()
        };
        for entity in pause_entities {
            change_visibilty(context, &entity);
        }
    }
}

fn set_debug_visibility(context: &Context) {
    let mut debug_query: Query = Query::new(&context.world).with::<DebugComponent>();
    let debug_entities: Vec<Entity> = debug_query.entities_with_components().unwrap();

    for debug_entity in &debug_entities {
        change_visibilty(context, debug_entity);
    }
}

fn change_visibilty(context: &Context, entity: &Entity) {
    let mut visibility_component: ComponentRefMut<'_, Visibility> = context.world.get_entity_component_mut(entity).unwrap();
    visibility_component.0 = !visibility_component.0;
}


pub fn spawn_menu(context: &mut Context) {
    let start_text: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), MENU_FONT_SIZE),
        Position::new(Vector2::new(MENU_TEXT_X, MENU_TEXT_Y), Strategy::Pixelated),
        Color::YELLOW,
        "> enter <".to_string()
    );
    let start_text_shadow: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), MENU_FONT_SIZE),
        Position::new(Vector2::new(MENU_TEXT_X+0.002, MENU_TEXT_Y+0.002), Strategy::Pixelated),
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
    let pause: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 40.0),
        Position::new(Vector2::new(PAUSE_TEXT_X, 0.07), Strategy::Pixelated),
        Color::WHITE,
        "pause".to_string()
    );
    let quit_menu: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 30.0),
        Position::new(Vector2::new(PAUSE_TEXT_X, PAUSE_TEXT_Y), Strategy::Pixelated),
        Color::WHITE,
        "quit to menu".to_string()
    );
    let quit: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 30.0),
        Position::new(Vector2::new(PAUSE_TEXT_X, PAUSE_TEXT_Y + 0.07), Strategy::Pixelated),
        Color::WHITE,
        "quit game".to_string()
    );
    let selection: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 30.0),
        Position::new(Vector2::new(PAUSE_TEXT_X-0.02, PAUSE_TEXT_Y), Strategy::Pixelated),
        Color::WHITE,
        ">".to_string()
    );
    context.commands.spawn(
        vec![
            Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK)),
            Box::new(Transform::new(
                Position::new(Vector2::new(1.3, 0.0), Strategy::Normalized),
                0.0,
                Scale::new(Vector2::new(0.6, 4.0), Strategy::Normalized))),
            Box::new(Pause()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER-1)),
        ]
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
            Box::new(quit_menu),
            Box::new(Pause()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(MENU_DRAW_ORDER))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(quit),
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

pub fn spawn_borders(context: &mut Context) {
    context.commands.spawn(
        vec![
            Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::RED)),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.5, 0.0), Strategy::Normalized),
                0.0,
                Scale::new(Vector2::new(0.01, 5.0), Strategy::Normalized))),
            Box::new(Border()),
            Box::new(DebugComponent()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(4)),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::RED)),
            Box::new(Transform::new(
                Position::new(Vector2::new(-0.5, 0.0), Strategy::Normalized),
                0.0,
                Scale::new(Vector2::new(0.01, 5.0), Strategy::Normalized))),
            Box::new(Border()),
            Box::new(DebugComponent()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(4)),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
}
