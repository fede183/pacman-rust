use bevy::prelude::*;
use crate::{components::Pacman, config::SQUARE_SIZE};
use crate::animation::animation_config::AnimationConfig;

pub fn init_stage(
    commands: Commands, 
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
    init_pacman(commands, asset_server, texture_atlas_layouts);
}

pub fn init_pacman(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) { 

    let texture = asset_server.load("images/pacman_sprite_sheet.png");
    // The sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(14), 6, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // The first (left-hand) sprite runs at 10 FPS
    let animation_config_1 = AnimationConfig::new(1, 6, 10);


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
          custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
          ..default()
        },
    ));
}
