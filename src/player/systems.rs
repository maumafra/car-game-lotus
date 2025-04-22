use lotus_engine::*;

use crate::player::components::*;

use crate::cars::components::OpponentCar;
use crate::common::components::Border;
use crate::common::resources::{GameAudio, GameState, GameStateEnum};
use crate::menus::components::GameOver;
use crate::menus::systems::change_menu_visibility;
use crate::score::systems::{save_highscore_time, reset_score};

const CAR_SPAWN_Y: f32 = -0.5;
const CAR_ROTATION: f32 = 10.0;
const CAR_CRASH_ROTATION: f32 = 30.0;

pub fn spawn_player(context: &mut Context) {
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/48x48/cars/white-lancer.png".to_string())),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.0, CAR_SPAWN_Y), Strategy::Normalized),
                0.0,
                Vector2::new(1.0, 1.0))),
            Box::new(MainCar()),
            Box::new(Velocity::new(Vector2::new(1.0, 1.0))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle))),
            Box::new(DrawOrder(3))
        ]
    );
}

pub fn horn(context: &Context) {
    let game_state: ResourceRef<'_, GameState> = context.world.get_resource::<GameState>().unwrap();
    if game_state.0 == GameStateEnum::Running {
        let mut game_audio: ResourceRefMut<'_, GameAudio> = context.world.get_resource_mut::<GameAudio>().unwrap();
        game_audio.0.play_static_sound("car_horn".to_string()).ok();
    }
}

pub fn reset_player(context: &Context) {
    let player_entity: Entity = {
        let mut player_entity_query: Query = Query::new(&context.world).with::<MainCar>();
        player_entity_query.entities_with_components().unwrap().first().unwrap().clone()
    };
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&player_entity).unwrap();
    transform.set_rotation(&context.render_state, 0.0);
    transform.set_position_x(&context.render_state, 0.0);
}

pub fn move_player(context: &Context, direction: f32) {
    let game_state: GameStateEnum = context.world.get_resource::<GameState>().unwrap().0.clone();
    if game_state != GameStateEnum::Running {
        return;
    }
    let player_entity: Entity = {
        let mut player_entity_query: Query = Query::new(&context.world).with::<MainCar>();
        player_entity_query.entities_with_components().unwrap().first().unwrap().clone()
    };
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&player_entity).unwrap();
    let car_speed: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&player_entity).unwrap();

    let movement: f32 = transform.position.x + direction * car_speed.x * context.delta;
    let rotation: f32 = - CAR_ROTATION * direction;

    transform.set_position_x(&context.render_state, movement);
    transform.set_rotation(&context.render_state, rotation);
}

pub fn move_crashed_player(context: &Context) {
    let player_entity: Entity = {
        let mut player_entity_query: Query = Query::new(&context.world).with::<MainCar>();
        player_entity_query.entities_with_components().unwrap().first().unwrap().clone()
    };
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&player_entity).unwrap();
    let rotation: f32  = transform.rotation + CAR_CRASH_ROTATION * context.delta;
    transform.set_rotation(&context.render_state, rotation);
}

pub fn check_player_collisions(context: &mut Context) {
    let player_entity: Entity = {
        let mut player_entity_query: Query = Query::new(&context.world).with::<MainCar>();
        player_entity_query.entities_with_components().unwrap().first().unwrap().clone()
    };
    if check_player_border_collision(context, player_entity) || check_player_opponent_collision(context, player_entity) {
        //crash(context);
    }
}

fn crash(context: &mut Context) {
    save_highscore_time(context);
    reset_score(context);
    let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
    game_state.0 = GameStateEnum::GameOver;
    change_menu_visibility::<GameOver>(context);
    let mut game_audio: ResourceRefMut<'_, GameAudio> = context.world.get_resource_mut::<GameAudio>().unwrap();
    game_audio.0.stop_streaming_sound("car_acceleration".to_string()).ok();
    //eprintln!("crash!");
}

fn check_player_border_collision(context: &mut Context, player_entity: Entity) -> bool {
    let borders_entities: Vec<Entity> = {
        let mut border_query: Query = Query::new(&context.world).with::<Border>();
        border_query.entities_with_components().unwrap()
    };

    let player_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&player_entity).unwrap();
    let mut collides: bool = false;

    for border in &borders_entities {
        let border_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(border).unwrap();
        if Collision::check(CollisionAlgorithm::Aabb, &player_collision, &border_collision) {
            eprintln!("crash on border!");
            collides = true;
            break;
        }
    }
    return collides;
}

fn check_player_opponent_collision(context: &Context, player_entity: Entity) -> bool {
    let mut opponents_query: Query = Query::new(&context.world).with::<OpponentCar>();
    let opponents_entities: Vec<Entity> = opponents_query.entities_with_components().unwrap();

    let player_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&player_entity).unwrap();
    let mut collides: bool = false;

    for opponent in &opponents_entities {
        let opponent_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(opponent).unwrap();

        if Collision::check(CollisionAlgorithm::Aabb, &player_collision, &opponent_collision) {
            eprintln!("crash on car!");
            collides = true;
            break;
        }
    }
    return collides;
}