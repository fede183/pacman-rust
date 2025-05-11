use bevy::state::state::{States, ComputedStates};

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Playing;

impl ComputedStates for Playing {
    type SourceStates = GameState;

    fn compute(sources: GameState) -> Option<Self> {
        match sources {
            GameState::Playing { .. } => Some(Playing),
            _ => None
        }
    }
}
