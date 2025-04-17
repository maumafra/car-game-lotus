use lotus_engine::*;
use std::time::Duration;

#[derive(Clone, Component)]
pub struct JustAComponent();

#[derive(Clone, Resource)]
pub struct JustAResource(u32);

#[derive(Clone, Component)]
pub struct MainCar();

#[derive(Clone, Component)]
pub struct MainCarSpeed(f32);

#[derive(Clone, Component)]
pub struct Car();

#[derive(Clone, Component)]
pub struct Border();

#[derive(Clone, Resource)]
pub struct CarSpawnTimer(pub Timer);

impl Default for CarSpawnTimer {
    fn default() -> Self {
        return Self(Timer::new(TimerType::Repeat, Duration::new(2, 0)))
    }
}

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let white_lancer_sprite: Sprite = Sprite::new("sprites/white-lancer.png".to_string());
    let red_car_sprite: Sprite = Sprite::new("sprites/red-car.png".to_string());
    let blue_car_sprite: Sprite = Sprite::new("sprites/blue-car.png".to_string());
    let yellow_car_sprite: Sprite = Sprite::new("sprites/yellow-car.png".to_string());

    let player_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::RED);

    let border_left_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK);
    let border_right_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK);

    context.commands.spawn(
        vec![
            Box::new(player_shape),
            Box::new(Transform::new(Vector2::new(0.0, -0.5), 0.0, Vector2::new(0.08, 0.4))),
            Box::new(MainCar()),
            Box::new(MainCarSpeed(1.0)),
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
    
}

fn move_player_car(context: &Context, player_entity: Entity, input: ResourceRef<'_, Input>) {
    let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&player_entity).unwrap();
    let car_speed: ComponentRef<'_, MainCarSpeed> = context.world.get_entity_component::<MainCarSpeed>(&player_entity).unwrap();

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyA)) {
        let move_left = transform.position.x - car_speed.0 * context.delta;
        transform.set_position_x(&context.render_state, move_left);
    }

    if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyD)) {
        let move_right = transform.position.x + car_speed.0 * context.delta;
        transform.set_position_x(&context.render_state, move_right);
    }
}