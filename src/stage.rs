use std::collections::LinkedList;

use bevy::prelude::*;
use crate::components::Limit;
use crate::{components::Pacman, config::SQUARE_SIZE};
use crate::animation::animation_config::AnimationConfig;
use crate::config::*;
use crate::sprites::rectagle::Rectangle;

pub fn init_stage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
    init_limit(&mut commands);
    init_pacman(&mut commands, asset_server, texture_atlas_layouts);
}

// This system runs when the user clicks the left arrow key or right arrow key
pub fn trigger_animation<S: Component>(mut animation: Single<&mut AnimationConfig, With<S>>) {
    // We create a new timer when the animation is triggered
    animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
}

pub fn init_pacman(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) { 

    let texture = asset_server.load("images/pacman_sprite_sheet.png");
    // The sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // The first (left-hand) sprite runs at 10 FPS
    let animation_config_1 = AnimationConfig::new(0, 1, 5);


    commands.spawn((
        Pacman,
        Transform {
            translation: Vec3::new(0., 0., 5.),
            ..default()
        },
        Sprite {
          image: texture.clone(),
          texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config_1.first_sprite_index,
          }),
          custom_size: Some(Vec2::new(SQUARE_SIZE / 2., SQUARE_SIZE / 2.)),
          ..default()
        },
        animation_config_1,
    ));
}

pub fn init_limit(
    commands: &mut Commands,
    ) { 
    let mut limits = LinkedList::new();
    let limit_border_left = Rectangle::new(LIMIT_HEIGHT, 3., HEADER_BORDER_COLOR);
    let limit_border_right = Rectangle::new(LIMIT_HEIGHT, 3., HEADER_BORDER_COLOR);
    let limit_border_up = Rectangle::new(3., LIMIT_WIGTH, HEADER_BORDER_COLOR);
    let limit_border_down = Rectangle::new(3., LIMIT_WIGTH, HEADER_BORDER_COLOR);

    let position_left = Vec3::new(-LIMIT_WIGTH / 2., LIMIT_POSITIONS.y, LIMIT_POSITIONS.z);
    let position_right = Vec3::new(LIMIT_WIGTH / 2., LIMIT_POSITIONS.y, LIMIT_POSITIONS.z);
    let position_up = Vec3::new(LIMIT_POSITIONS.x, SQUARE_SIZE * 3.5, LIMIT_POSITIONS.z);
    let position_down = Vec3::new(LIMIT_POSITIONS.x, -SQUARE_SIZE * 5.5, LIMIT_POSITIONS.z);

    limits.push_front((limit_border_left, position_left));
    limits.push_front((limit_border_right, position_right));
    limits.push_front((limit_border_up, position_up));
    limits.push_front((limit_border_down, position_down));

    for limit in limits {
        commands.spawn((
            Limit,
            limit.0.generate_sprite(limit.1)
        ));
    }
}
