use bevy::prelude::*;
use crate::{components::Pacman, config::SQUARE_SIZE};

pub fn init_stage(
    commands: Commands, 
    asset_server: Res<AssetServer>,
    ) {
    init_pacman(commands, asset_server);
}

pub fn init_pacman(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    ) { 

    let texture = asset_server.load("images/pacman_closemounth.png");

    commands.spawn((
        Pacman,
        Transform::from_xyz(SQUARE_SIZE, SQUARE_SIZE, 0.),
        Sprite {
          image: texture,
          ..default()
        },
    ));
}
