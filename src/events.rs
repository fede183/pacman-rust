use bevy::prelude::*;
use crate::animation::animation_config::AnimationConfig;
use crate::game::game_data::GameData;
use crate::config::SQUARE_SIZE;
use crate::game::game_state::GameState;
use crate::utils::cycle_timer::CycleTimer;
use crate::utils::event_blocker::EventBlocker;
use crate::components::{Wall, Pacman};

pub fn input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut event_blocker: ResMut<EventBlocker>,
    time: ResMut<Time>,
    mut query_pacman_transformation: Query<&mut Transform, With<Pacman>>,
    query_limit_transformation: Query<&Transform, With<Wall>>,
    ) {
    
    if !(key_pressed(&input, KeyCode::ArrowDown) || 
        key_pressed(&input, KeyCode::ArrowUp) || 
        key_pressed(&input, KeyCode::ArrowLeft) || 
        key_pressed(&input, KeyCode::ArrowRight)) {
        return;
    }

    if query_pacman_transformation.is_empty() {
        return;
    }

    if !event_blocker.check(time.clone()) {
        return;
    }
    
    event_blocker.lock_process();

    let mut transform = query_pacman_transformation.single_mut();
    let movement_distance = SQUARE_SIZE / 3.;

    let mut transform_pacman_temp = transform.clone();

    if key_pressed(&input, KeyCode::ArrowDown) {
        transform_pacman_temp.translation.y -= movement_distance;
        transform_pacman_temp.rotation = Quat::from_rotation_z(270.0_f32.to_radians());
    }
    if key_pressed(&input, KeyCode::ArrowUp) {
        transform_pacman_temp.translation.y += movement_distance;
        transform_pacman_temp.rotation = Quat::from_rotation_z(90.0_f32.to_radians());
    }
    if key_pressed(&input, KeyCode::ArrowLeft) {
        transform_pacman_temp.translation.x -= movement_distance;
        transform_pacman_temp.rotation = Quat::from_rotation_y(180.0_f32.to_radians());
    }
    if key_pressed(&input, KeyCode::ArrowRight) {
        transform_pacman_temp.translation.x += movement_distance;
        transform_pacman_temp.rotation = Quat::from_rotation_y(0.0_f32.to_radians());
    }

    for transform_limit in query_limit_transformation.iter() {
        let distance = transform_pacman_temp.translation.distance(transform_limit.translation);
        let pacman_radious = SQUARE_SIZE / 2.;
        let limit_radious = 3.;
        if distance < pacman_radious + limit_radious {
            println!("Collition detected");
            event_blocker.finish_process();
            return;
        }
    }
    
    transform.translation = transform_pacman_temp.translation;
    transform.rotation = transform_pacman_temp.rotation;

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
                }
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}


pub fn cycle_system(
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

