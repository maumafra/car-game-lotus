use lotus_engine::*;

use crate::background::components::*;
use crate::background::resources::*;

const BACKGROUND_VELOCITY: f32 = -3.0;

pub fn spawn_background_tiles(context: &mut Context) {
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/960x600/worlds/night-bridge_0.png".to_string())),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.005, 0.0), Strategy::Normalized),
                0.0,
                Vector2::new(1.55, 1.0))),
            Box::new(Background()),
            Box::new(DrawOrder(0))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/960x600/worlds/night-bridge_0.png".to_string())),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.005, 2.0), Strategy::Normalized),
                0.0,
                Vector2::new(1.55, 1.0))),
            Box::new(Background()),
            Box::new(DrawOrder(0))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(Sprite::new("sprites/960x600/worlds/night-bridge_1.png".to_string())),
            Box::new(Transform::new(
                Position::new(Vector2::new(0.005, 4.0), Strategy::Normalized),
                0.0,
                Vector2::new(1.55, 1.0))),
            Box::new(Background()),
            Box::new(DrawOrder(0))
        ]
    );
}

pub fn reset_background(context: &Context) {
    let background_entities: Vec<Entity> = {
        let mut background_query: Query = Query::new(&context.world).with::<Background>();
        background_query.entities_with_components().unwrap()
    };
    let tile_counter: ResourceRef<'_, BackgroundTileCounter> = context.world.get_resource::<BackgroundTileCounter>().unwrap();
    let mut tile_index: usize = tile_counter.0 as usize;
    let mut y: f32 = 0.0;
    for _ in vec![1, 2, 3] {
        let bg_entity: Entity = background_entities.get(tile_index).unwrap().clone();
        let mut bg_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&bg_entity).unwrap();
        bg_transform.set_position_y(&context.render_state, y);
        y += 2.0;
        tile_index = (tile_index+1)%3;
    }
}

pub fn handle_background_tiles(context: &Context) {
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

pub fn move_background(context: &Context) {
    let background_entities: Vec<Entity> = {
        let mut background_query: Query = Query::new(&context.world).with::<Background>();
        background_query.entities_with_components().unwrap()
    };

    for backgound in background_entities {
        let mut bg_transform: ComponentRefMut<'_, Transform> = context.world.get_entity_component_mut::<Transform>(&backgound).unwrap();

        let move_down: f32 = bg_transform.get_position().y + BACKGROUND_VELOCITY * context.delta;
        bg_transform.set_position_y(&context.render_state, move_down);
    }
}