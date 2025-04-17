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
        return Self(Timer::new(TimerType::Repeat, Duration::new(2, 0)))
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
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let white_lancer_sprite: Sprite = Sprite::new("sprites/64x64/white-lancer.png".to_string());

    let border_left_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK);
    let border_right_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK);

    context.commands.add_resources(vec![
        Box::new(CarSpawnTimer::default()),
        Box::new(CarSprites::default())
    ]);

    context.commands.spawn(
        vec![
            Box::new(white_lancer_sprite),
            Box::new(Transform::new(Vector2::new(0.0, -0.5), 0.0, Vector2::new(0.1, 0.1))),
            Box::new(MainCar()),
            Box::new(Velocity::new(Vector2::new(1.0, 1.0))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(border_left_shape),
            Box::new(Transform::new(Vector2::new(0.8, 0.0), 0.0, Vector2::new(0.01, 5.0))),
            Box::new(Border()),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );

    context.commands.spawn(
        vec![
            Box::new(border_right_shape),
            Box::new(Transform::new(Vector2::new(-0.8, 0.0), 0.0, Vector2::new(0.01, 5.0))),
            Box::new(Border()),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
}

fn update(context: &mut Context) {
    let input: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();

    let mut player_entity_query: Query = Query::new(&context.world).with::<MainCar>();
    let player_entity: Entity = player_entity_query.entities_with_components().unwrap().first().unwrap().clone();

    move_player_car(context, player_entity, input);
    check_player_collision(context, player_entity);
    spawn_opponent_cars(context);
}

fn move_player_car(context: &Context, player_entity: Entity, input: ResourceRef<'_, Input>) {
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
            println!("colisão");
        }
    }
}

fn spawn_opponent_cars(context: &Context) {
    let mut car_spawn_timer: ResourceRefMut<'_, CarSpawnTimer> = context.world.get_resource_mut::<CarSpawnTimer>().unwrap();
    
    car_spawn_timer.0.tick(context.delta);

    if car_spawn_timer.0.is_finished() {
        let mut thread_rng: ThreadRng = rand::rng();
        let random_factor: i32 = thread_rng.random_range(30..=100);
        let number_of_opponents_to_spawn: i32 = random_factor/30;
        let car_sprites: ResourceRef<'_, CarSprites> = context.world.get_resource::<CarSprites>().unwrap();

        for _ in 1..=number_of_opponents_to_spawn {
            let lane_number: u32 = thread_rng.random_range(1..=4);
            let car_sprite: &String = car_sprites.0.choose(&mut thread_rng).unwrap();
            spawn_opponent_car(context, lane_number, car_sprite);
        }
    }
}

fn spawn_opponent_car(context: Context, lane_number: u32, car_srpite_path: &String) {
    let lane_number_float: f32 = f32::from_bits(lane_number);
    context.commands.spawn(
        vec![
            Box::new(Sprite::new(car_srpite_path.to_string())),
            Box::new(Transform::new(Vector2::new((1.6/lane_number_float) - 0.8, 3.0), 0.0, Vector2::new(0.1, 0.1))),
            Box::new(OpponentCar()),
            Box::new(Velocity::new(Vector2::new(0.4, 0.4))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
}