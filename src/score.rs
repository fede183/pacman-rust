use bevy::prelude::*;
use crate::config::*;
use crate::game::game_data::GameData;
use crate::sprites::rectagle::RectangleWithBorder;

#[derive(Component)]
pub struct ScoreComponent;

#[derive(Component)]
pub struct ClockComponent;

pub fn init_header(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/textFont.ttf");
    let text_color = TextColor(Color::from(SCORE_COLOR));
    let text_font = TextFont {
        font: font.clone(),
        font_size: SCORE_FONT_SIZE,
        ..default()
    };
    let text_justification = JustifyText::Center;

    let score_text = "Score: 0".to_owned();
    let clock_text = "Timer: 00:00:00".to_owned();
    
    let text_score = Text2d::new(score_text);
    let text_clock = Text2d::new(clock_text);
    
    let transform_score = Transform::from_xyz(DISPLAY_SCORE_POSITION_X, DISPLAY_SCORE_POSITION_Y, 4.);

    commands.spawn((ScoreComponent,
            text_score,
            text_color,
            text_font.clone(),
            TextLayout::new_with_justify(text_justification),
            transform_score
    ));

    let transform_clock = Transform::from_xyz(DISPLAY_CLOCK_POSITION_X, DISPLAY_CLOCK_POSITION_Y, 4.);

    commands.spawn((ClockComponent,
            text_clock,
            text_color,
            text_font.clone(),
            TextLayout::new_with_justify(text_justification),
            transform_clock
    ));

    let header = RectangleWithBorder::new(HEADER_HEIGHT, HEADER_WIGTH, HEADER_BORDER_SIZE, HEADER_FILL_COLOR, HEADER_BORDER_COLOR);
    let positions = HEADER_POSITIONS;
    header.spawn(&mut commands, positions);
}

pub fn update_score(
    game_data: ResMut<GameData>,
    mut query_score: Query<&mut Text2d, With<ScoreComponent>>,
) {

    let score_text = "Score: ".to_owned() + &game_data.score.to_string();

    let mut text_score_component = query_score.single_mut();

    text_score_component.0 = score_text;
}
