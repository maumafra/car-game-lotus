use lotus_engine::*;

use crate::common::components::*;
use crate::common::resources::*;
use crate::menus::systems::*;
use crate::menus::components::*;
use crate::background::systems::reset_background;
use crate::cars::systems::reset_cars;
use crate::player::systems::move_player;
use crate::player::systems::reset_player;
use crate::score::systems::reset_score;
use crate::score::systems::resume_score;
use crate::score::systems::start_score;

pub fn handle_input(context: &mut Context) {
    let input: Input = {
        context.world.get_resource::<Input>().unwrap().clone()
    };

    if input.is_key_released(PhysicalKey::Code(KeyCode::Enter)) {
        start_game(context);
        enter_pause_option(context);
        enter_game_over_option(context);
    }

    if input.is_key_released(PhysicalKey::Code(KeyCode::Escape)) {
        toggle_pause(context);
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
        move_game_over_selection(context, -1);
    }
    if input.is_key_released(PhysicalKey::Code(KeyCode::KeyS)) {
        move_pause_selection(context, 1);
        move_game_over_selection(context, 1);
    }

    if input.is_key_released(PhysicalKey::Code(KeyCode::Semicolon)) {
        set_debug_visibility(context);
    }
}

pub fn quit_game() {
    std::process::exit(0);
}

pub fn reset_game(context: &mut Context) {
    reset_background(context);
    reset_score(context);
    reset_player(context);
    reset_cars(context);
}

pub fn retry(context: &Context) {
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    game_state.0 = GameStateEnum::Running;
}

pub fn quit_to_menu(context: &Context) {
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    game_state.0 = GameStateEnum::Menu;
}

fn start_game(context: &Context) {
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    if game_state.0 == GameStateEnum::Menu {
        game_state.0 = GameStateEnum::Running;

        change_menu_visibility::<Menu>(context);
        start_score(context);
    }
}

fn toggle_pause(context: &Context) {
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    if game_state.0 == GameStateEnum::Running {
        game_state.0 = GameStateEnum::Paused;

        change_menu_visibility::<Pause>(context);
    } else if game_state.0 == GameStateEnum::Paused {
        game_state.0 = GameStateEnum::Running;
        resume_score(context);

        change_menu_visibility::<Pause>(context);
    }
}

fn set_debug_visibility(context: &Context) {
    let mut debug_query: Query = Query::new(&context.world).with::<DebugComponent>();
    let debug_entities: Vec<Entity> = debug_query.entities_with_components().unwrap();

    for debug_entity in &debug_entities {
        change_visibilty(context, debug_entity);
    }
}

pub fn change_visibilty(context: &Context, entity: &Entity) {
    let mut visibility_component: ComponentRefMut<'_, Visibility> = context.world.get_entity_component_mut(entity).unwrap();
    visibility_component.0 = !visibility_component.0;
}

pub fn spawn_borders(context: &mut Context) {
    context.commands.spawn(
        vec![
            Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::RED)),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.5, 0.0), Strategy::Normalized),
                0.0,
                Vector2::new(0.01, 5.0))),
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
                Vector2::new(0.01, 5.0))),
            Box::new(Border()),
            Box::new(DebugComponent()),
            Box::new(Visibility(false)),
            Box::new(DrawOrder(4)),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
}
