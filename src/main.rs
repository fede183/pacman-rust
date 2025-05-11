mod stage;
mod config;
mod score;
mod events;
mod game;
mod utils;
mod plugins;
mod sprites;
mod components;

use bevy::prelude::*;
use game::game_state::{GameState, Playing};
use plugins::{game_over_plugin::GameOverPlugin, in_game_plugin::InGamePlugin};


fn main() {
    App::new()
        .add_plugins(InGamePlugin {
            state: GameState::Playing,
        })
        .add_plugins(GameOverPlugin {
            state: GameState::GameOver,
        })
        .init_state::<GameState>()
        .add_computed_state::<Playing>()
        .run();
}

