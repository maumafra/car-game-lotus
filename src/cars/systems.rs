use lotus_engine::*;
use rand::{rngs::ThreadRng, seq::IndexedRandom, Rng};

use crate::cars::components::*;
use crate::cars::resources::*;

const CAR_DESPAWN: f32 = -1.3;
const MAX_SPAWN_VALUE: i32 = 5;
const MIN_SPAWN_VALUE: i32 = 1;
const SPAWN_OFFSET: f32 = 0.9;
const SPAWN_RANGE: f32 = 0.75;

pub fn reset_cars(context: &mut Context) {
    let opponents_entities: Vec<Entity> = {
        let mut opponents_query: Query = Query::new(&context.world).with::<OpponentCar>();
        opponents_query.entities_with_components().unwrap()
    };
    if opponents_entities.is_empty() {
        return;
    }
    for opponent in opponents_entities {
        context.commands.despawn(opponent);
    }
}

pub fn handle_cars_movement(context: &mut Context) {
    let opponents_entities: Vec<Entity> = {
        let mut opponents_query: Query = Query::new(&context.world).with::<OpponentCar>();
        opponents_query.entities_with_components().unwrap()
    };

    for opponent in opponents_entities {
        let mut op_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&opponent).unwrap();

        if op_transform.get_position().y < CAR_DESPAWN {
            //eprintln!("opponent despawned: {:?}", opponent.0);
            context.commands.despawn(opponent);
        } else {
            let op_velocity: ComponentRef<'_, Velocity> = context.world.get_entity_component::<Velocity>(&opponent).unwrap();
            let move_down: f32 = op_transform.position.y + op_velocity.y * context.delta;
            op_transform.set_position_y(&context.render_state, move_down);
        }
    }
}

pub fn spawn_cars(context: &mut Context) {
    let timer_finished: bool = {
        let mut car_spawn_timer_ref: ResourceRefMut<'_, CarSpawnTimer> = context.world.get_resource_mut::<CarSpawnTimer>().unwrap();
        car_spawn_timer_ref.0.tick(context.delta);
        car_spawn_timer_ref.0.is_finished()
    };

    if timer_finished {
        let mut thread_rng: ThreadRng = rand::rng();
        let number_of_opponents_to_spawn: i32 = thread_rng.random_range(MIN_SPAWN_VALUE..=MAX_SPAWN_VALUE);
        let car_sprites: CarSprites = {
            let car_sprites_ref: ResourceRef<'_, CarSprites> = context.world.get_resource::<CarSprites>().unwrap();
            car_sprites_ref.clone()
        };

        //eprintln!("number of opponents to spawn: {:?}", number_of_opponents_to_spawn);
        
        let mut lane_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        for _ in 1..=number_of_opponents_to_spawn {
            let lane_index: usize = thread_rng.random_range(0..lane_numbers.len()) as usize;
            let lane_number: i32 = lane_numbers.remove(lane_index);
            //eprintln!("lane_number: {:?}",lane_number);

            let car_sprite: &String = car_sprites.0.choose(&mut thread_rng).unwrap();
            let car_offset: f32 = thread_rng.random_range(-SPAWN_OFFSET..=SPAWN_OFFSET);
            let spawn_position: Vector2<f32> = calculate_car_spawn_position(lane_number, car_offset);
            spawn_car(context, spawn_position, car_sprite);
        }
    }
}

fn spawn_car(context: &mut Context, spawn_position: Vector2<f32>, car_srpite_path: &String) {
    //eprintln!("opponent spawned at: {:?},{:?}", spawn_position.x, spawn_position.y);
    context.commands.spawn(
        vec![
            Box::new(Sprite::new(car_srpite_path.to_string())),
            Box::new(Transform::new(
                Position::new(spawn_position, Strategy::Normalized),
                0.0,
                Scale::new(Vector2::new(0.08, 0.08), Strategy::Normalized))),
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
    let spawn_range: f32 = SPAWN_RANGE;

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