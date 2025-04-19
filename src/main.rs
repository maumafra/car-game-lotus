use lotus_engine::core::ecs::{query, world};
use lotus_engine::*;
use rand::{rngs::ThreadRng, seq::IndexedRandom, Rng};
use std::time::{Duration, Instant};
use std::vec;

#[derive(Component)]
pub struct MainCar();

#[derive(Component)]
pub struct OpponentCar();

#[derive(Component)]
pub struct Border();

#[derive(Component)]
pub struct DebugComponent();

#[derive(Component)]
pub struct Background();

#[derive(Component)]
pub struct TimeComponent();

#[derive(Component)]
pub struct Menu();
#[derive(Component)]
pub struct GameRunning();

#[derive(Resource)]
pub struct CarSpawnTimer(pub Timer);

impl Default for CarSpawnTimer {
    fn default() -> Self {
        return Self(Timer::new(TimerType::Repeat, Duration::new(1, 0)))
    }
}

#[derive(Resource)]
pub struct ScoreTime {
    start_time: Instant,
    current_time: Duration,
    paused: bool,
}

impl Default for ScoreTime {
    fn default() -> Self {
        return Self{
            start_time: Instant::now(),
            current_time: Duration::ZERO,
            paused: false
        }
    }
}

#[derive(Resource)]
pub struct Highscore(pub Duration);

#[derive(Clone, Resource)]
pub struct GameState(pub GameStateEnum);

impl Default for GameState {
    fn default() -> Self {
        return Self(GameStateEnum::Menu);
    }
}

#[derive(Clone, PartialEq)]
pub enum GameStateEnum {
    Menu,
    Paused,
    Running
}

#[derive(Clone, Resource)]
pub struct CarSprites(pub Vec<String>);

impl Default for CarSprites {
    fn default() -> Self {
        return Self(Vec::from(vec![
            "sprites/64x64/cars/red-car.png".to_string(),
            "sprites/64x64/cars/blue-car.png".to_string(),
            "sprites/64x64/cars/yellow-car.png".to_string()
        ]))
    }
}

#[derive(Resource)]
pub struct BackgroundTileCounter(pub i32);

your_game!(
    WindowConfiguration {
        icon_path: "sprites/64x64/cars/white-lancer.png".to_string(),
        title: "Lancer TURBO: Fury on the Road".to_string(),
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
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/960x600/menu.png".to_string())),
            Box::new(Transform::new(Vector2::new(0.0, 0.0), 0.0, Vector2::new(1.6, 1.0))),
            Box::new(Menu()),
            Box::new(DrawOrder(10))
        ]
    );

    let white_lancer_sprite: Sprite = Sprite::new("sprites/64x64/cars/white-lancer.png".to_string());

    context.commands.add_resources(vec![
        Box::new(GameState::default()),
        Box::new(ScoreTime::default()),
        Box::new(Highscore(Duration::ZERO)),
        Box::new(CarSpawnTimer::default()),
        Box::new(CarSprites::default()),
        Box::new(BackgroundTileCounter(0))
    ]);

    // Player
    context.commands.spawn(
        vec![
            Box::new(white_lancer_sprite),
            Box::new(Transform::new(Vector2::new(0.0, -0.5), 0.0, Vector2::new(0.08, 0.08))),
            Box::new(MainCar()),
            Box::new(Velocity::new(Vector2::new(1.0, 1.0))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle))),
            Box::new(DrawOrder(3))
        ]
    );

    // Background Tiles
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/960x600/worlds/night-bridge.png".to_string())),
            Box::new(Transform::new(Vector2::new(0.005, 0.0), 0.0, Vector2::new(1.55, 1.0))),
            Box::new(Background()),
            Box::new(DrawOrder(0))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/960x600/worlds/night-bridge.png".to_string())),
            Box::new(Transform::new(Vector2::new(0.005, 2.0), 0.0, Vector2::new(1.55, 1.0))),
            Box::new(Background()),
            Box::new(DrawOrder(0))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/960x600/worlds/night-bridge.png".to_string())),
            Box::new(Transform::new(Vector2::new(0.005, 4.0), 0.0, Vector2::new(1.55, 1.0))),
            Box::new(Background()),
            Box::new(DrawOrder(0))
        ]
    );

    // Borders
    context.commands.spawn(
        vec![
            Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::RED)),
            Box::new(Transform::new(Vector2::new(0.5, 0.0), 0.0, Vector2::new(0.01, 5.0))),
            Box::new(Border()),
            Box::new(DebugComponent()),
            Box::new(Visibility{value: false}),
            Box::new(DrawOrder(4)),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::RED)),
            Box::new(Transform::new(Vector2::new(-0.5, 0.0), 0.0, Vector2::new(0.01, 5.0))),
            Box::new(Border()),
            Box::new(DebugComponent()),
            Box::new(Visibility{value: false}),
            Box::new(DrawOrder(4)),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle)))
        ]
    );

    // Score screen
    let score_text: Text = Text::new(
        //Font::new(Fonts::UnderdogRegular.get_path(), 40.0),
        Font::new("fonts/x12y16pxMaruMonica.ttf".to_string(), 40.0),
        Vector2::new(0.07, 0.07),
        Color::WHITE,
        "SCORE".to_string()
    );
    let score_time: Text = Text::new(
        //Font::new(Fonts::UnderdogRegular.get_path(), 40.0),
        Font::new("fonts/x12y16pxMaruMonica.ttf".to_string(), 40.0),
        Vector2::new(0.053, 0.14),
        Color::WHITE,
        "00:00:000".to_string()
    );
    let highscore_text: Text = Text::new(
        //Font::new(Fonts::UnderdogRegular.get_path(), 40.0),
        Font::new("fonts/x12y16pxMaruMonica.ttf".to_string(), 40.0),
        Vector2::new(0.04, 0.28),
        Color::ORANGE,
        "HIGHSCORE".to_string()
    );
    let highscore_time: Text = Text::new(
        //Font::new(Fonts::UnderdogRegular.get_path(), 40.0),
        Font::new("fonts/x12y16pxMaruMonica.ttf".to_string(), 40.0),
        Vector2::new(0.053, 0.35),
        Color::ORANGE,
        "00:00:000".to_string()
    );
    context.commands.spawn(
        vec![
            Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK)),
            Box::new(Transform::new(Vector2::new(-1.3, 0.0), 0.0, Vector2::new(0.6, 4.0))),
            Box::new(DrawOrder(4))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(score_text),
            Box::new(DrawOrder(5))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(score_time),
            Box::new(TimeComponent()),
            Box::new(DrawOrder(5))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(highscore_text),
            Box::new(DrawOrder(5))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(highscore_time),
            Box::new(TimeComponent()),
            Box::new(DrawOrder(5))
        ]
    );
    
}

fn update(context: &mut Context) {
    let player_entity: Entity = {
        let mut player_entity_query: Query = Query::new(&context.world).with::<MainCar>();
        player_entity_query.entities_with_components().unwrap().first().unwrap().clone()
    };

    handle_input(context);

    if context.world.get_resource::<GameState>().unwrap().0 == GameStateEnum::Running {
        move_player(context, player_entity);
        update_score_time(context);
        handle_background_tiles(context);
        move_background(context);
        handle_opponents_movement(context);
        check_player_collisions(context, player_entity);
        spawn_opponent(context);
    }
}

fn update_score_time(context: &mut Context) {
    let mut score_time: ResourceRefMut<ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
    if score_time.paused {
       return; 
    }
    
    let time_entity: Entity = {
        let mut time_entity_query: Query = Query::new(&context.world).with::<TimeComponent>();
        time_entity_query.entities_with_components().unwrap().first().unwrap().clone()
    };

    score_time.current_time = score_time.start_time.elapsed();
    let millis: u128 = score_time.current_time.as_millis()%1000;
    let seconds: u32 = ((score_time.current_time.as_millis()/1000)%60) as u32;
    let minutes: u32 = (score_time.current_time.as_millis()/60000) as u32;

    context.render_state.text_renderers.get_mut(&time_entity.0).unwrap().update_brush(
        (format!("{:02}:{:02}:{:03}", minutes, seconds, millis)).to_string(),
        context.render_state.queue.clone(),
        context.render_state.physical_size
    );
}

fn save_highscore_time(context: &mut Context) {
    let score_time: ResourceRef<ScoreTime> = context.world.get_resource::<ScoreTime>().unwrap();
    let mut highscore_time: ResourceRefMut<Highscore> = context.world.get_resource_mut::<Highscore>().unwrap();

    let highscore_entity: Entity = {
        let mut highscore_entity_query: Query = Query::new(&context.world).with::<TimeComponent>();
        highscore_entity_query.entities_with_components().unwrap().last().unwrap().clone()
    };
    let highscore: Duration = score_time.start_time.elapsed();
    if highscore_time.0 < highscore{
        highscore_time.0 = highscore;
        let millis: u128 = highscore.as_millis()%1000;
        let seconds: u32 = ((highscore.as_millis()/1000)%60) as u32;
        let minutes: u32 = (highscore.as_millis()/60000) as u32;

        context.render_state.text_renderers.get_mut(&highscore_entity.0).unwrap().update_brush(
            (format!("{:02}:{:02}:{:03}", minutes, seconds, millis)).to_string(),
            context.render_state.queue.clone(),
            context.render_state.physical_size
        );
    }   
}

fn handle_input(context: &Context) {
    let input: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();

    if input.is_key_released(PhysicalKey::Code(KeyCode::Enter)) {
        let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
        let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
        if game_state.0 == GameStateEnum::Menu {
            game_state.0 = GameStateEnum::Running;

            let mut menu_query: Query =  Query::new(&context.world).with::<Menu>();
            let menu_entity: Entity = menu_query.entities_with_components().unwrap().first().unwrap().clone();
            change_visibilty(context, &menu_entity);

            score_time.start_time = Instant::now() - score_time.current_time;
        }
    }

    if input.is_key_released(PhysicalKey::Code(KeyCode::Escape)) {
        let mut game_state: ResourceRefMut<'_, GameState> = context.world.get_resource_mut::<GameState>().unwrap();
        let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
        if game_state.0 == GameStateEnum::Running {
            game_state.0 = GameStateEnum::Paused;
            score_time.paused = true;
        } else if game_state.0 == GameStateEnum::Paused {
            game_state.0 = GameStateEnum::Running;
            score_time.paused = false;
            score_time.start_time = Instant::now() - score_time.current_time;
        }
    }
    set_debug_visibility(context, &input);
}

fn handle_background_tiles(context: &Context) {
    let background_entities: Vec<Entity> = {
        let mut background_query: Query = Query::new(&context.world).with::<Background>();
        background_query.entities_with_components().unwrap()
    };

    let mut tile_counter: ResourceRefMut<'_, BackgroundTileCounter> = context.world.get_resource_mut::<BackgroundTileCounter>().unwrap();

    let first_tile_index: usize = tile_counter.0 as usize;
    let last_tile_intex: usize = (first_tile_index + 2)%3;

    let first_background_entity: Entity =  background_entities.get(first_tile_index).unwrap().clone();
    let last_background_entity: Entity =  background_entities.get(last_tile_intex).unwrap().clone();

    let mut f_bg_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&first_background_entity).unwrap();
    let l_bg_transform: ComponentRef<'_, Transform> = context.world.get_entity_component::<Transform>(&last_background_entity).unwrap();

    if f_bg_transform.get_position().y <= -2.0 {
        let new_y: f32 = 2.0 + l_bg_transform.get_position().y;
        f_bg_transform.set_position_y(&context.render_state, new_y);
        tile_counter.0 = (tile_counter.0+1)%3;
    }
}

fn move_background(context: &Context) {
    let background_entities: Vec<Entity> = {
        let mut background_query: Query = Query::new(&context.world).with::<Background>();
        background_query.entities_with_components().unwrap()
    };

    for backgound in background_entities {
        let mut bg_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&backgound).unwrap();

        let move_down: f32 = bg_transform.get_position().y - 3.0 * context.delta;
        bg_transform.set_position_y(&context.render_state, move_down);
    }
}

fn move_player(context: &Context, player_entity: Entity) {
    let input: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&player_entity).unwrap();
    let car_speed: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&player_entity).unwrap();

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyA)) {
        let move_left: f32 = transform.position.x - car_speed.value.x * context.delta;
        transform.set_position_x(&context.render_state, move_left);
        transform.set_rotation(&context.render_state, 10.0);
    }

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyD)) {
        let move_right: f32 = transform.position.x + car_speed.value.x * context.delta;
        transform.set_position_x(&context.render_state, move_right);
        transform.set_rotation(&context.render_state, -10.0);
    }

    if input.is_key_released(PhysicalKey::Code(KeyCode::KeyA))
    || input.is_key_released(PhysicalKey::Code(KeyCode::KeyD)) {
        transform.set_rotation(&context.render_state, 0.0);
    }
    
}

fn set_debug_visibility(context: &Context, input: &ResourceRef<'_, Input>) {
    // Alterar a visibilidade das entidades com o componente debug
    if input.is_key_released(PhysicalKey::Code(KeyCode::Semicolon)) {
        let mut debug_query: Query = Query::new(&context.world).with::<DebugComponent>();
        let debug_entities: Vec<Entity> = debug_query.entities_with_components().unwrap();
    
        for debug_entity in &debug_entities {
            change_visibilty(context, debug_entity);
        }
    }
}

fn change_visibilty(context: &Context, entity: &Entity) {
    let mut visibility_component: ComponentRefMut<'_, Visibility> = context.world.get_entity_component_mut(entity).unwrap();
    visibility_component.value = !visibility_component.value;
}

fn check_player_collisions(context: &mut Context, player_entity: Entity) {
    if check_player_border_collision(context, player_entity) || check_player_opponent_collision(context, player_entity) {
        eprintln!("crash!");
        save_highscore_time(context);
        reset_score(context);
    }
}

fn reset_score(context: &Context) {
    let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
    score_time.start_time = Instant::now();
    score_time.current_time = Duration::ZERO;
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
            collides = true;
            break;
        }
    }
    return collides;
}

fn handle_opponents_movement(context: &mut Context) {
    let opponents_entities: Vec<Entity> = {
        let mut opponents_query: Query = Query::new(&context.world).with::<OpponentCar>();
        opponents_query.entities_with_components().unwrap()
    };

    for opponent in opponents_entities {
        let mut op_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&opponent).unwrap();

        if op_transform.get_position().y < -1.3 {
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
        let mut thread_rng: ThreadRng = rand::rng();
        let number_of_opponents_to_spawn: i32 = thread_rng.random_range(1..=5);
        let car_sprites: CarSprites = {
            let car_sprites_ref: ResourceRef<'_, CarSprites> = context.world.get_resource::<CarSprites>().unwrap();
            car_sprites_ref.clone()
        };

        eprintln!("number of opponents to spawn: {:?}", number_of_opponents_to_spawn);
        
        let mut lane_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        for _ in 1..=number_of_opponents_to_spawn {
            let lane_index: usize = thread_rng.random_range(0..lane_numbers.len()) as usize;
            let lane_number: i32 = lane_numbers.remove(lane_index);
            eprintln!("lane_number: {:?}",lane_number);

            let car_sprite: &String = car_sprites.0.choose(&mut thread_rng).unwrap();
            let car_offset: f32 = thread_rng.random_range(-0.9..=0.9);
            let spawn_position: Vector2<f32> = calculate_car_spawn_position(lane_number, car_offset);
            spawn_opponent_car(context, spawn_position, car_sprite);
        }
    }
}

fn spawn_opponent_car(context: &mut Context, spawn_position: Vector2<f32>, car_srpite_path: &String) {
    eprintln!("opponent spawned at: {:?},{:?}", spawn_position.x, spawn_position.y);
    context.commands.spawn(
        vec![
            Box::new(Sprite::new(car_srpite_path.to_string())),
            Box::new(Transform::new(spawn_position, 0.0, Vector2::new(0.08, 0.08))),
            Box::new(OpponentCar()),
            Box::new(Velocity::new(Vector2::new(0.0, -2.0))),
            Box::new(Collision::new(Collider::new_simple(GeometryType::Rectangle))),
            Box::new(DrawOrder(2))
        ]
    );
}

fn calculate_car_spawn_position(lane_number: i32, car_offset: f32) -> Vector2<f32> {
    let lane_number_float: f32 = lane_number as f32;
    // spawn_range represents the total range in x axis which cars can be spawned,
    // for example: with spawn_range = 0.8, a car can be spawned between x = -0.4 and x = 0.4
    let spawn_range: f32 = 0.75;

    // f(n) = an + b
    // with 'n' as lane number (which varies from 1..6)
    // so in the first lane (n=1), the car would be spawned on the left border (negative part on x axis), having:
    // 1*a + b = (-1.0)*(spawn_range/2.0)
    // 
    // and in the last lane (n=6), the car would be spawned on the right border (positive part on x axis), having:
    // 6*a + b = (spawn_range/2.0)
    // 
    // so we get:
    // | a + b = -spawn_range/2.0
    // | 6a + b = spawn_range/2.0
    // with some function manipulation, we get: 5a = spawn_range
    let a: f32 = spawn_range/5.0;
    let b: f32 = spawn_range/(-2.0) - a;

    return Vector2::new(a*lane_number_float + b, 3.0 + car_offset);
}