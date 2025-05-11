use bevy::prelude::*;
use crate::config::*;
use crate::sprites::rectagle::RectangleWithBorder;
use crate::game::game_data::GameData;

pub struct GameOverPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for GameOverPlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(self.state.clone()), init_game_over_windows);
    } 
}

fn init_game_over_windows(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    game_data: ResMut<GameData>,
    ) {
    RectangleWithBorder::new(DISPLAY_GAME_OVER_WINDOW_WIGTH, DISPLAY_GAME_OVER_WINDOW_HEIGHT, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR).spawn(&mut commands, Vec3{ x: 0., y: 0., z: 7.});
    
    let font = asset_server.load("fonts/textFont.ttf");
    let text_style = TextFont {
        font: font.clone(),
        font_size: SCORE_FONT_SIZE,
        ..default()
    };
    let game_over_text = "Game Over!!!".to_owned();

    commands.spawn((
            Text2d::new(game_over_text),
            TextLayout::new_with_justify(JustifyText::Center),
            text_style.clone(),
            Transform::from_xyz(0., 10., 8.),
            TextColor(SCORE_COLOR.into()),
        )
    );

    let final_score_text = "Score: ".to_owned() + &game_data.score.to_string();

    commands.spawn(
        (
            Text2d::new(final_score_text),
            TextLayout::new_with_justify(JustifyText::Center),
            text_style.clone(),
            Transform::from_xyz(0., -10., 8.),
            TextColor(SCORE_COLOR.into()),
        )
    );

    // TODO: add buttons for exit and replay
}
