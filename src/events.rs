use bevy::prelude::*;
use crate::animation::animation_config::AnimationConfig;
use crate::game::game_data::GameData;
use crate::config::SQUARE_SIZE;
use crate::game::game_state::GameState;
use crate::utils::cycle_timer::CycleTimer;
use crate::utils::event_blocker::EventBlocker;
use crate::components::Pacman;

pub fn input_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut event_blocker: ResMut<EventBlocker>,
    time: ResMut<Time>,
    mut query_pacman_transformation: Query<(Entity, &mut Transform), With<Pacman>>,
    ) {
    
    if !(key_pressed(&input, KeyCode::ArrowDown) || 
        key_pressed(&input, KeyCode::ArrowLeft) || 
        key_pressed(&input, KeyCode::ArrowRight) ||
        key_pressed(&input, KeyCode::Space)) {
        return;
    }

    if query_pacman_transformation.is_empty() {
        return;
    }

    if !event_blocker.check(time.clone()) {
        return;
    }
    
    event_blocker.lock_process();

    let (entity, mut transform) = query_pacman_transformation.single_mut();
    let movement_distance = SQUARE_SIZE / 3.;

    if key_pressed(&input, KeyCode::ArrowDown) {
        transform.translation.y -= movement_distance;
    }
    if key_pressed(&input, KeyCode::ArrowLeft) {
        transform.translation.x -= movement_distance;
    }
    if key_pressed(&input, KeyCode::ArrowRight) {
        transform.translation.x += movement_distance;
    }

    event_blocker.finish_process();
}


// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations_pacman(
    time: Res<Time>, 
    mut query: Query<(&mut AnimationConfig, &mut Sprite), With<Pacman>>,
) {
    for (mut config, mut sprite) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_sprite_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}


pub fn cycle_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut cycle_system: ResMut<CycleTimer>,
    mut event_blocker: ResMut<EventBlocker>,
    time: ResMut<Time>,
    mut query_pacman_transformation: Query<(Entity, &mut Transform), With<Pacman>>,
    ) {

    if !event_blocker.check(time.clone()) {
        return;
    }

    if !cycle_system.check(time.clone()) {
        return;
    }

    let (entity, mut _transform) = query_pacman_transformation.single_mut();
}

pub fn toggle_game_over(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_data: ResMut<GameData>,
) {
    if *state.get() == GameState::Playing && game_data.is_game_over() {
        next_state.set(GameState::GameOver);
    }
}

fn key_pressed(
    input: &Res<ButtonInput<KeyCode>>,
    key_code: KeyCode
    ) -> bool {
    input.just_pressed(key_code) || input.pressed(key_code)
}

