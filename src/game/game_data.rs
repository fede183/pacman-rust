use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct GameData {
    pub score: i32,
}

impl GameData {
    pub fn new_game() -> GameData {
        let score = 0;

        GameData { score }
    }

    pub fn is_game_over(&self) -> bool {
        false
    }
}
