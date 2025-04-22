use lotus_engine::*;
use std::time::{Duration, Instant};

use crate::common::resources::MARU_MONICA_FONT_PATH;
use crate::score::components::*;
use crate::score::resources::*;


const TEXT_DRAW_ORDER: u32 = 5;

pub fn spawn_score_screen(context: &mut Context) {
    let score_text: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 40.0),
        Position::new(Vector2::new(0.07, 0.07), Strategy::Pixelated),
        Color::WHITE,
        "score".to_string()
    );
    let score_time: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 40.0),
        Position::new(Vector2::new(0.053, 0.14), Strategy::Pixelated),
        Color::WHITE,
        "00:00:000".to_string()
    );
    let highscore_text: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 40.0),
        Position::new(Vector2::new(0.016, 0.28), Strategy::Pixelated),
        Color::ORANGE,
        "> highscore <".to_string()
    );
    let highscore_time: Text = Text::new(
        Font::new(MARU_MONICA_FONT_PATH.to_string(), 40.0),
        Position::new(Vector2::new(0.053, 0.35), Strategy::Pixelated),
        Color::ORANGE,
        "00:00:000".to_string()
    );
    context.commands.spawn(
        vec![
            Box::new(Shape::new(Orientation::Horizontal, GeometryType::Rectangle, Color::BLACK)),
            Box::new(Transform::new(
                Position::new(Vector2::new(-1.3, 0.0), Strategy::Normalized),
                0.0,
                Vector2::new(0.6, 4.0))),
            Box::new(DrawOrder(TEXT_DRAW_ORDER-1))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(score_text),
            Box::new(DrawOrder(TEXT_DRAW_ORDER))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(score_time),
            Box::new(TimeComponent()),
            Box::new(DrawOrder(TEXT_DRAW_ORDER))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(highscore_text),
            Box::new(DrawOrder(TEXT_DRAW_ORDER))
        ]
    );
    context.commands.spawn(
        vec![
            Box::new(highscore_time),
            Box::new(TimeComponent()),
            Box::new(DrawOrder(TEXT_DRAW_ORDER))
        ]
    );
}

pub fn update_score_time(context: &mut Context) {
    let current_time: Duration = {
        let mut score_time: ResourceRefMut<ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
        score_time.current_time = score_time.start_time.elapsed();
        score_time.current_time
    };

    let time_entity: Entity = {
        let mut time_entity_query: Query = Query::new(&context.world).with::<TimeComponent>();
        time_entity_query.entities_with_components().unwrap().first().unwrap().clone()
    };

    let millis: u128 = current_time.as_millis()%1000;
    let seconds: u32 = ((current_time.as_millis()/1000)%60) as u32;
    let minutes: u32 = (current_time.as_millis()/60000) as u32;

    context.world.text_renderers.get_mut(&time_entity.0).unwrap().update_brush(
        (format!("{:02}:{:02}:{:03}", minutes, seconds, millis)).to_string(),
        context.render_state.queue.clone(),
        context.render_state.physical_size
    );
}

pub fn save_highscore_time(context: &mut Context) {
    let highscore_entity: Entity = {
        let mut highscore_entity_query: Query = Query::new(&context.world).with::<TimeComponent>();
        highscore_entity_query.entities_with_components().unwrap().last().unwrap().clone()
    };
    let finished_score: Duration = {
        let score_time: ResourceRef<ScoreTime> = context.world.get_resource::<ScoreTime>().unwrap();
        score_time.start_time.elapsed()
    };
    let update_high_score: bool = {
        let mut highscore_time: ResourceRefMut<Highscore> = context.world.get_resource_mut::<Highscore>().unwrap();
        if highscore_time.0 < finished_score {
            highscore_time.0 = finished_score;
        }
        highscore_time.0 == finished_score
    };
    if update_high_score {
        let millis: u128 = finished_score.as_millis()%1000;
        let seconds: u32 = ((finished_score.as_millis()/1000)%60) as u32;
        let minutes: u32 = (finished_score.as_millis()/60000) as u32;

        context.world.text_renderers.get_mut(&highscore_entity.0).unwrap().update_brush(
            (format!("{:02}:{:02}:{:03}", minutes, seconds, millis)).to_string(),
            context.render_state.queue.clone(),
            context.render_state.physical_size
        );
    }   
}

pub fn reset_score(context: &Context) {
    let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
    score_time.start_time = Instant::now();
    score_time.current_time = Duration::ZERO;
}

pub fn start_score(context: &Context) {
    let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
    score_time.start_time = Instant::now() - score_time.current_time;
}

pub fn resume_score(context: &Context) {
    let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
    score_time.start_time = Instant::now() - score_time.current_time;
}