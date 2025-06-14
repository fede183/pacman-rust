use bevy::prelude::*;
use crate::components::Pacman;
use crate::stage::init_stage;
use crate::stage::trigger_animation;
use crate::utils::cycle_timer::*;
use crate::utils::event_blocker::EventBlocker;
use crate::events::*;
use crate::score::*;
use crate::config::*;
use crate::game::game_data::GameData;

pub struct InGamePlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for InGamePlugin<S> {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameData::new_game())
            .insert_resource(ClearColor(Color::BLACK))
            .init_resource::<CycleTimer>()
            .init_resource::<EventBlocker>()
            .add_plugins(DefaultPlugins)
            .add_systems(Startup, (init_camera, toggle_window, init_stage, init_header))
            .add_systems(Update, (update_score, input_system, cycle_system, toggle_game_over, execute_animations_pacman)
            .run_if(in_state(self.state.clone())));
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn toggle_window(mut window: Query<&mut Window>) {
    let mut window_mut = window.single_mut();
    window_mut.title = "Pacman!".into();
    window_mut.name = Some("ingame.app".into());
    window_mut.resolution = (DISPLAY_WINDOW_WIGTH, DISPLAY_WINDOW_HEIGHT).into();
    window_mut.prevent_default_event_handling = false;
    window_mut.visible = true;
}

fn setup_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioPlayer::new(asset_server.load("sounds/theme.wav")));
}

