use lotus_engine::*;

#[derive(Clone, Component)]
pub struct JustAComponent();

#[derive(Clone, Resource)]
pub struct JustAResource(u32);

#[derive(Clone, Component)]
pub struct RaceCar();

#[derive(Clone, Component)]
pub struct Car();

#[derive(Clone, Resource)]
pub struct CarSpawner(pub Timer);

//impl Default for CarSpawner {
//    fn default() -> Self {
//        return Self(Timer::new(TimerType::Repeat, Duration::new(2, 0)))
//    }
//}

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    // Shape => Component
    let white_lancer_sprite: Sprite = Sprite::new("sprites/white-lancer.png".to_string());
    let red_car_sprite: Sprite = Sprite::new("sprites/red-car.png".to_string());
    let blue_car_sprite: Sprite = Sprite::new("sprites/blue-car.png".to_string());
    let yellow_car_sprite: Sprite = Sprite::new("sprites/yellow-car.png".to_string());

    context.commands.spawn(
        vec![
            Box::new(white_lancer_sprite),
            Box::new(Transform::new(Vector2::new(0.0, 0.0), 0.0, Vector2::new(0.1, 0.1)))
        ]
    )

    //let my_shape: Shape = Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::RED);

    // Transform::new(Vector2: positsion, Int: rotation, Vector2: scale)
    
    //let transform: Transform = Transform::new(Vector2::new(0.0, 0.0), 0.0, Vector2::new(0.25, 0.25));

    //context.commands.spawn(vec![Box::new(my_shape), Box::new(transform), Box::new(JustAComponent())]);

    //context.commands.add_resource(Box::new(JustAResource(1)));
}

fn update(context: &mut Context) {
    // Just to demonstrate, you can set more filters to your query.
    // In this case only passing Shape would do the trick.
    // But our entity have the Transform component too, so it will work as well.
    //let mut query: Query =  Query::new(&context.world).with::<JustAComponent>();
    //let my_entity: Entity = query.entities_with_components().unwrap().first().unwrap().clone();
    //let mut transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&my_entity).unwrap();
//
    //let input: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();
//
    //let mut just_a_resource: ResourceRefMut<'_, JustAResource> = context.world.get_resource_mut::<JustAResource>().unwrap();
//
    //if input.is_key_pressed(PhysicalKey::Code(KeyCode::KeyX)) {
    //    let my_rotation: f32 = transform.rotation + 100.0 * context.delta;
    //    transform.set_rotation(&context.render_state, my_rotation);
    //}
//
    //if input.is_key_released(PhysicalKey::Code(KeyCode::KeyX)) {
    //    just_a_resource.0 += 1;
    //    eprintln!("Resource Value: {:?}", just_a_resource.0);
    //}
}