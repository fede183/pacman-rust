use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::Wall;
use crate::components::Pacman;
use crate::animation::animation_config::AnimationConfig;
use crate::config::*;
use crate::config_walls::*;

pub fn init_stage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
    init_wall(&mut commands);
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
            translation: Vec3::new(0., 0., 2.),
            ..default()
        },
        Sprite {
          image: texture.clone(),
          texture_atlas: Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: animation_config_1.first_sprite_index,
          }),
          custom_size: Some(Vec2::new(PACMAN_SIZE, PACMAN_SIZE)),
          ..default()
        },
        animation_config_1,
        KinematicCharacterController::default(),
        Collider::cuboid(PACMAN_SIZE/2., PACMAN_SIZE/2.),
        Sensor::default(),
    ));
}

pub fn init_wall(
    commands: &mut Commands,
    ) { 

    let walls = get_wall_bundles();

    for wall in walls {
        if let Some(custom_size) = wall.sprite.custom_size { 
            commands.spawn((
                Wall,
                wall,
                Collider::cuboid(custom_size.x/2., custom_size.y/2.),
            ));
        }
    }
}
