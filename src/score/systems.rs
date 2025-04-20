use lotus_engine::*;
use std::time::{Duration, Instant};

use crate::score::components::*;
use crate::score::resources::*;

pub fn spawn_score_screen(context: &mut Context) {
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

pub fn update_score_time(context: &mut Context) {
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

pub fn save_highscore_time(context: &mut Context) {
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

pub fn reset_score(context: &Context) {
    let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
    score_time.start_time = Instant::now();
    score_time.current_time = Duration::ZERO;
}

pub fn start_score(context: &Context) {
    let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
    score_time.start_time = Instant::now() - score_time.current_time;
}

pub fn pause_score(context: &Context) {
    let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
    score_time.paused = true;
}

pub fn resume_score(context: &Context) {
    let mut score_time: ResourceRefMut<'_, ScoreTime> = context.world.get_resource_mut::<ScoreTime>().unwrap();
    score_time.paused = false;
    score_time.start_time = Instant::now() - score_time.current_time;
}