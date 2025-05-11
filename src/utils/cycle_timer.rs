use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct CycleTimer {
    timer: Timer,
}

impl CycleTimer {
    pub fn new() -> Self {
        Self { timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating) }
    }

    pub fn check(&mut self, time: Time) -> bool {
        self.timer.tick(time.delta());

        return self.timer.finished() && self.timer.just_finished();
    }
}

impl Default for CycleTimer {
    fn default() -> Self {
        Self::new()
    }
}
