use lotus_engine::*;
use rand::{rngs::ThreadRng, seq::IndexedRandom, Rng};
use std::{time::Duration, vec};

#[derive(Clone, Component)]
pub struct MainCar();

#[derive(Clone, Component)]
pub struct OpponentCar();

#[derive(Clone, Component)]
pub struct Border();

#[derive(Clone, Resource)]
pub struct CarSpawnTimer(pub Timer);

impl Default for CarSpawnTimer {
    fn default() -> Self {
        return Self(Timer::new(TimerType::Repeat, Duration::new(1, 0)))
    }
}

#[derive(Clone, Resource)]
pub struct CarSprites(pub Vec<String>);

impl Default for CarSprites {
    fn default() -> Self {
        return Self(Vec::from(vec![
            "sprites/64x64/red-car.png".to_string(),
            "sprites/64x64/blue-car.png".to_string(),
            "sprites/64x64/yellow-car.png".to_string()
        ]))
    }
}

your_game!(
    WindowConfiguration {
        icon_path: "sprites/64x64/white-lancer.png".to_string(),
        title: "Scarlet Eyes: Fury on the Road".to_string(),
        background_color: Some(Color::GRAY),
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
    let white_lancer_sprite: Sprite = Sprite::new("sprites/64x64/white-lancer.png".to_string());

    let border_left_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK);
    let border_right_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK);

    //let test: Shape = Shape::new(Orientation::Horizontal, GeometryType::Square, Color::RED);
    //context.commands.spawn(
    //    vec![
    //        Box::new(test),
    //        Box::new(Transform::new(Vector2::new(0.0, 1.0), 0.0, Vector2::new(0.01, 0.01)))
    //    ]
    //);

    context.commands.add_resources(vec![
        Box::new(CarSpawnTimer::default()),
        Box::new(CarSprites::default())
    ]);

    context.commands.spawn(
        vec![
            Box::new(white_lancer_sprite),
            Box::new(Transform::new(Vector2::new(0.0, -0.5), 0.0, Vector2::new(0.08, 0.08))),
            Box::new(MainCar()),
            Box::new(Velocity::new(Vector2::new(1.0, 1.0))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(border_left_shape),
            Box::new(Transform::new(Vector2::new(0.5, 0.0), 0.0, Vector2::new(0.01, 5.0))),
            Box::new(Border()),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(border_right_shape),
            Box::new(Transform::new(Vector2::new(-0.5, 0.0), 0.0, Vector2::new(0.01, 5.0))),
            Box::new(Border()),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
}

fn update(context: &mut Context) {
    let input: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();

    let mut player_entity_query: Query = Query::new(&context.world).with::<MainCar>();
    let player_entity: Entity = player_entity_query.entities_with_components().unwrap().first().unwrap().clone();

    let mut opponents_query: Query = Query::new(&context.world).with::<OpponentCar>();
    let opponents_entities: Vec<Entity> = opponents_query.entities_with_components().unwrap();

    move_player(context, player_entity, input);
    check_player_collision(context, player_entity);
    handle_opponents_movement(context, opponents_entities);
    spawn_opponent(context);
}

fn move_player(context: &Context, player_entity: Entity, input: ResourceRef<'_, Input>) {
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&player_entity).unwrap();
    let car_speed: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&player_entity).unwrap();

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyA)) {
        let move_left: f32 = transform.position.x - car_speed.value.x * context.delta;
        transform.set_position_x(&context.render_state, move_left);
    }

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyD)) {
        let move_right: f32 = transform.position.x + car_speed.value.x * context.delta;
        transform.set_position_x(&context.render_state, move_right);
    }
}

fn check_player_collision(context: &Context, player_entity: Entity) {
    let mut border_query: Query = Query::new(&context.world).with::<Border>();
    let borders_entities: Vec<Entity> = border_query.entities_with_components().unwrap();

    let player_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(&player_entity).unwrap();

    for border in &borders_entities {
        let border_collision: ComponentRef<'_, Collision> = context.world.get_entity_component::<Collision>(border).unwrap();

        if Collision::check(CollisionAlgorithm::Aabb, &player_collision, &border_collision) {
            eprintln!("crash!");
        }
    }
}

fn handle_opponents_movement(context: &mut Context, opponents_entities: Vec<Entity>) {
    for opponent in opponents_entities {
        let mut op_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&opponent).unwrap();

        if op_transform.get_position().y < -1.5 {
            eprintln!("opponent despawned: {:?}", opponent.0);
            context.commands.despawn(opponent);
        } else {
            let op_velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&opponent).unwrap();
            let move_down: f32 = op_transform.position.y + op_velocity.value.y * context.delta;
            op_transform.set_position_y(&context.render_state, move_down);
        }
    }
}

fn spawn_opponent(context: &mut Context) {
    let timer_finished: bool = {
        let mut car_spawn_timer_ref: ResourceRefMut<'_, CarSpawnTimer> = context.world.get_resource_mut::<CarSpawnTimer>().unwrap();
        car_spawn_timer_ref.0.tick(context.delta);
        car_spawn_timer_ref.0.is_finished()
    };

    if timer_finished {
        // Testing timer.
        eprintln!("timers up");

        let mut thread_rng: ThreadRng = rand::rng();
        let number_of_opponents_to_spawn: i32 = thread_rng.random_range(1..=3);
        let car_sprites: CarSprites = {
            let car_sprites_ref: ResourceRef<'_, CarSprites> = context.world.get_resource::<CarSprites>().unwrap();
            car_sprites_ref.clone()
        };

        eprintln!("number of opponents to spawn: {:?}", number_of_opponents_to_spawn);
        
        let mut lane_numbers: Vec<i32> = vec![1, 2, 3, 4];
        for _ in 1..=number_of_opponents_to_spawn {
            let lane_index: usize = thread_rng.random_range(0..lane_numbers.len()) as usize;
            let lane_number: i32 = lane_numbers.remove(lane_index);
            eprintln!("lane_number: {:?}",lane_number);

            let car_sprite: &String = car_sprites.0.choose(&mut thread_rng).unwrap();
            let car_offset: f32 = thread_rng.random_range(-0.9..=0.9);
            spawn_opponent_car(context, lane_number, car_sprite, car_offset);
        }
    }
}

fn spawn_opponent_car(context: &mut Context, lane_number: i32, car_srpite_path: &String, car_offset: f32) {
    let lane_number_float: f32 = lane_number as f32;
    let x_position: f32 = (0.7/3.0)*lane_number_float - (7.0/12.0);
    let y_position: f32 = 3.0 + car_offset;
    eprintln!("opponent spawned at: {:?},{:?}", x_position, y_position);
    context.commands.spawn(
        vec![
            Box::new(Sprite::new(car_srpite_path.to_string())),
            Box::new(Transform::new(Vector2::new(x_position, y_position), 0.0, Vector2::new(0.08, 0.08))),
            Box::new(OpponentCar()),
            Box::new(Velocity::new(Vector2::new(0.0, -2.0))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
}