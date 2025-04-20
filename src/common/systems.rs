use lotus_engine::*;

use crate::common::components::*;
use crate::common::resources::*;
use crate::score::systems::resume_score;
use crate::score::systems::start_score;

pub fn spawn_menu(context: &mut Context) {
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/960x600/menu.png".to_string())),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
                0.0,
                Scale::new(Vector2::new(1.6, 1.0), Strategy::Normalized))),
            Box::new(Menu()),
            Box::new(DrawOrder(10))
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

pub fn handle_input(context: &Context) {
    let input: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();

    if input.is_key_released(PhysicalKey::Code(KeyCode::Enter)) {
        start_game(context);
    }

    if input.is_key_released(PhysicalKey::Code(KeyCode::Escape)) {
        toggle_pause(context);
    }

    if input.is_key_released(PhysicalKey::Code(KeyCode::Semicolon)) {
        set_debug_visibility(context);
    }
}

fn start_game(context: &Context) {
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    if game_state.0 == GameStateEnum::Menu {
        game_state.0 = GameStateEnum::Running;
        let menu_entity: Entity = {
            let mut menu_query: Query =  Query::new(&context.world).with::<Menu>();
            menu_query.entities_with_components().unwrap().first().unwrap().clone()
        };
        change_visibilty(context, &menu_entity);
        start_score(context);
    }
}

fn toggle_pause(context: &Context) {
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    if game_state.0 == GameStateEnum::Running {
        game_state.0 = GameStateEnum::Paused;
    } else if game_state.0 == GameStateEnum::Paused {
        game_state.0 = GameStateEnum::Running;
        resume_score(context);
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